//! LLVM Code Generation for Secondlang
//!
//! This module generates LLVM IR from the typed AST using inkwell.

use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::{IntPredicate, OptimizationLevel};

use crate::ast::{BinaryOp, Expr, Program, Stmt, TypedExpr, UnaryOp};
use crate::types::Type;

// ANCHOR: codegen_struct
/// Code generator state
pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    /// Map from variable names to their stack allocations
    variables: HashMap<String, PointerValue<'ctx>>,
    /// Map from function names to LLVM functions
    functions: HashMap<String, FunctionValue<'ctx>>,
    /// Current function being compiled
    current_fn: Option<FunctionValue<'ctx>>,
}
// ANCHOR_END: codegen_struct

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_fn: None,
        }
    }

    // ANCHOR: compile
    /// Compile a program and return the module
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        // First pass: declare all functions
        for stmt in program {
            if let Stmt::Function {
                name,
                params,
                return_type,
                ..
            } = stmt
            {
                self.declare_function(name, params, return_type)?;
            }
        }

        // Second pass: compile function bodies only (not top-level expressions)
        for stmt in program {
            if let Stmt::Function { .. } = stmt {
                self.compile_stmt(stmt)?;
            }
        }

        // Third pass: create __main wrapper for top-level expression
        if let Some(Stmt::Expr(expr)) = program.last() {
            self.compile_main_wrapper(expr)?;
        }

        // Verify module
        self.module
            .verify()
            .map_err(|e| format!("Module verification failed: {}", e.to_string()))?;

        Ok(())
    }
    // ANCHOR_END: compile

    /// Create a __main wrapper function for top-level expression
    fn compile_main_wrapper(&mut self, expr: &TypedExpr) -> Result<(), String> {
        // Create __main function: fn() -> i64
        let ret_type = self.context.i64_type();
        let fn_type = ret_type.fn_type(&[], false);
        let function = self.module.add_function("__main", fn_type, None);

        // Create entry block
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        self.current_fn = Some(function);

        // Compile the expression and return its value
        let value = self.compile_expr(expr)?;
        self.builder.build_return(Some(&value)).unwrap();

        Ok(())
    }

    /// Declare a function (without body)
    fn declare_function(
        &mut self,
        name: &str,
        params: &[(String, Type)],
        return_type: &Type,
    ) -> Result<FunctionValue<'ctx>, String> {
        let ret_type = self.llvm_type(return_type)?;
        let param_types: Vec<BasicMetadataTypeEnum> = params
            .iter()
            .map(|(_, t)| self.llvm_type(t).unwrap().into())
            .collect();

        let fn_type = ret_type.fn_type(&param_types, false);
        let function = self.module.add_function(name, fn_type, None);

        // Set parameter names
        for (i, (param_name, _)) in params.iter().enumerate() {
            function
                .get_nth_param(i as u32)
                .unwrap()
                .set_name(param_name);
        }

        self.functions.insert(name.to_string(), function);
        Ok(function)
    }

    /// Get LLVM type for our type
    fn llvm_type(&self, ty: &Type) -> Result<inkwell::types::IntType<'ctx>, String> {
        match ty {
            Type::Int => Ok(self.context.i64_type()),
            Type::Bool => Ok(self.context.bool_type()),
            Type::Unit => Ok(self.context.i64_type()), // Use i64 for unit
            Type::Unknown => Ok(self.context.i64_type()), // Default to i64
            Type::Function { .. } => Err("Cannot get LLVM type for function type".to_string()),
        }
    }

    /// Compile a statement
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<Option<IntValue<'ctx>>, String> {
        match stmt {
            Stmt::Function {
                name, params, body, ..
            } => {
                let function = self
                    .functions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Function {} not declared", name))?;

                // Create entry block
                let entry = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry);

                // Save current function
                self.current_fn = Some(function);
                self.variables.clear();

                // Allocate parameters
                for (i, (param_name, param_type)) in params.iter().enumerate() {
                    let param_value = function.get_nth_param(i as u32).unwrap().into_int_value();
                    let alloca =
                        self.create_entry_block_alloca(&function, param_name, param_type)?;
                    self.builder.build_store(alloca, param_value).unwrap();
                    self.variables.insert(param_name.clone(), alloca);
                }

                // Compile body
                let mut last_value = None;
                for body_stmt in body {
                    last_value = self.compile_stmt(body_stmt)?;
                }

                // Add return if needed
                if self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_terminator()
                    .is_none()
                {
                    if let Some(val) = last_value {
                        self.builder.build_return(Some(&val)).unwrap();
                    } else {
                        let zero = self.context.i64_type().const_int(0, false);
                        self.builder.build_return(Some(&zero)).unwrap();
                    }
                }

                Ok(None)
            }

            Stmt::Return(expr) => {
                let value = self.compile_expr(expr)?;
                self.builder.build_return(Some(&value)).unwrap();
                Ok(Some(value))
            }

            Stmt::Assignment { name, value, .. } => {
                let val = self.compile_expr(value)?;

                // Check if variable exists
                if let Some(ptr) = self.variables.get(name) {
                    self.builder.build_store(*ptr, val).unwrap();
                } else {
                    // Create new variable
                    let function = self.current_fn.unwrap();
                    let alloca = self.create_entry_block_alloca(&function, name, &value.ty)?;
                    self.builder.build_store(alloca, val).unwrap();
                    self.variables.insert(name.clone(), alloca);
                }

                Ok(Some(val))
            }

            Stmt::Expr(expr) => {
                let val = self.compile_expr(expr)?;
                Ok(Some(val))
            }
        }
    }

    // ANCHOR: compile_expr
    /// Compile an expression
    fn compile_expr(&mut self, expr: &TypedExpr) -> Result<IntValue<'ctx>, String> {
        match &expr.expr {
            Expr::Int(n) => Ok(self.context.i64_type().const_int(*n as u64, false)),

            Expr::Bool(b) => Ok(self.context.bool_type().const_int(*b as u64, false)),

            Expr::Var(name) => {
                let ptr = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))?;
                let val = self
                    .builder
                    .build_load(self.context.i64_type(), *ptr, name)
                    .unwrap();
                Ok(val.into_int_value())
            }

            Expr::Unary { op, expr: inner } => {
                let val = self.compile_expr(inner)?;
                match op {
                    UnaryOp::Neg => Ok(self.builder.build_int_neg(val, "neg").unwrap()),
                    UnaryOp::Not => Ok(self.builder.build_not(val, "not").unwrap()),
                }
            }

            Expr::Binary { op, left, right } => {
                let l = self.compile_expr(left)?;
                let r = self.compile_expr(right)?;

                match op {
                    BinaryOp::Add => Ok(self.builder.build_int_add(l, r, "add").unwrap()),
                    BinaryOp::Sub => Ok(self.builder.build_int_sub(l, r, "sub").unwrap()),
                    BinaryOp::Mul => Ok(self.builder.build_int_mul(l, r, "mul").unwrap()),
                    BinaryOp::Div => Ok(self.builder.build_int_signed_div(l, r, "div").unwrap()),
                    BinaryOp::Mod => Ok(self.builder.build_int_signed_rem(l, r, "mod").unwrap()),
                    BinaryOp::Lt => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SLT, l, r, "lt")
                            .unwrap();
                        Ok(self
                            .builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap())
                    }
                    BinaryOp::Gt => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SGT, l, r, "gt")
                            .unwrap();
                        Ok(self
                            .builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap())
                    }
                    BinaryOp::Le => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SLE, l, r, "le")
                            .unwrap();
                        Ok(self
                            .builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap())
                    }
                    BinaryOp::Ge => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SGE, l, r, "ge")
                            .unwrap();
                        Ok(self
                            .builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap())
                    }
                    BinaryOp::Eq => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::EQ, l, r, "eq")
                            .unwrap();
                        Ok(self
                            .builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap())
                    }
                    BinaryOp::Ne => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::NE, l, r, "ne")
                            .unwrap();
                        Ok(self
                            .builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap())
                    }
                }
            }

            Expr::Call { name, args } => {
                let function = self
                    .functions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined function: {}", name))?;

                let arg_values: Vec<BasicMetadataValueEnum> = args
                    .iter()
                    .map(|a| self.compile_expr(a).map(|v| v.into()))
                    .collect::<Result<_, _>>()?;

                let call = self
                    .builder
                    .build_call(function, &arg_values, "call")
                    .unwrap();
                Ok(call.try_as_basic_value().unwrap_basic().into_int_value())
            }

            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.compile_expr(cond)?;
                // Convert to i1 for branch
                let cond_bool = self
                    .builder
                    .build_int_truncate(cond_val, self.context.bool_type(), "cond")
                    .unwrap();

                let function = self.current_fn.unwrap();
                let then_bb = self.context.append_basic_block(function, "then");
                let else_bb = self.context.append_basic_block(function, "else");
                let merge_bb = self.context.append_basic_block(function, "merge");

                self.builder
                    .build_conditional_branch(cond_bool, then_bb, else_bb)
                    .unwrap();

                // Then branch
                self.builder.position_at_end(then_bb);
                let mut then_val = self.context.i64_type().const_int(0, false);
                for stmt in then_branch {
                    if let Some(v) = self.compile_stmt(stmt)? {
                        then_val = v;
                    }
                }
                let then_end = self.builder.get_insert_block().unwrap();
                let then_has_terminator = then_end.get_terminator().is_some();
                if !then_has_terminator {
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                // Else branch
                self.builder.position_at_end(else_bb);
                let mut else_val = self.context.i64_type().const_int(0, false);
                for stmt in else_branch {
                    if let Some(v) = self.compile_stmt(stmt)? {
                        else_val = v;
                    }
                }
                let else_end = self.builder.get_insert_block().unwrap();
                let else_has_terminator = else_end.get_terminator().is_some();
                if !else_has_terminator {
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                // Merge - only if at least one branch reaches it
                if then_has_terminator && else_has_terminator {
                    // Both branches return/terminate, merge block is unreachable
                    // Remove it and return a dummy value
                    unsafe {
                        merge_bb.delete().unwrap();
                    }
                    // Return a dummy value - the actual return happened in the branches
                    Ok(self.context.i64_type().const_int(0, false))
                } else {
                    self.builder.position_at_end(merge_bb);
                    let phi = self
                        .builder
                        .build_phi(self.context.i64_type(), "phi")
                        .unwrap();

                    // Only add incoming from branches that don't have terminators
                    if !then_has_terminator {
                        phi.add_incoming(&[(&then_val, then_end)]);
                    }
                    if !else_has_terminator {
                        phi.add_incoming(&[(&else_val, else_end)]);
                    }

                    Ok(phi.as_basic_value().into_int_value())
                }
            }

            Expr::While { cond, body } => {
                let function = self.current_fn.unwrap();
                let cond_bb = self.context.append_basic_block(function, "while_cond");
                let body_bb = self.context.append_basic_block(function, "while_body");
                let end_bb = self.context.append_basic_block(function, "while_end");

                self.builder.build_unconditional_branch(cond_bb).unwrap();

                // Condition
                self.builder.position_at_end(cond_bb);
                let cond_val = self.compile_expr(cond)?;
                let cond_bool = self
                    .builder
                    .build_int_truncate(cond_val, self.context.bool_type(), "cond")
                    .unwrap();
                self.builder
                    .build_conditional_branch(cond_bool, body_bb, end_bb)
                    .unwrap();

                // Body
                self.builder.position_at_end(body_bb);
                for stmt in body {
                    self.compile_stmt(stmt)?;
                }
                if self
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_terminator()
                    .is_none()
                {
                    self.builder.build_unconditional_branch(cond_bb).unwrap();
                }

                // End
                self.builder.position_at_end(end_bb);
                Ok(self.context.i64_type().const_int(0, false))
            }

            Expr::Block(stmts) => {
                let mut last_val = self.context.i64_type().const_int(0, false);
                for stmt in stmts {
                    if let Some(v) = self.compile_stmt(stmt)? {
                        last_val = v;
                    }
                }
                Ok(last_val)
            }
        }
    }
    // ANCHOR_END: compile_expr

    /// Create an alloca in the entry block
    fn create_entry_block_alloca(
        &self,
        function: &FunctionValue<'ctx>,
        name: &str,
        ty: &Type,
    ) -> Result<PointerValue<'ctx>, String> {
        let builder = self.context.create_builder();
        let entry = function.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(inst) => builder.position_before(&inst),
            None => builder.position_at_end(entry),
        }

        let llvm_type = self.llvm_type(ty)?;
        Ok(builder.build_alloca(llvm_type, name).unwrap())
    }

    /// Get the compiled module
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Print LLVM IR to string
    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

// ANCHOR: jit_run
/// JIT compile and run a program
pub fn jit_run(program: &Program) -> Result<i64, String> {
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "secondlang");

    codegen.compile(program)?;

    // Create execution engine
    let engine = codegen
        .module
        .create_jit_execution_engine(OptimizationLevel::Default)
        .map_err(|e| format!("Failed to create JIT: {}", e.to_string()))?;

    // Call the __main wrapper function which contains the top-level expression
    unsafe {
        let func: inkwell::execution_engine::JitFunction<unsafe extern "C" fn() -> i64> =
            engine.get_function("__main").map_err(|e| e.to_string())?;
        Ok(func.call())
    }
}
// ANCHOR_END: jit_run

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::typeck::typecheck;

    #[test]
    fn test_compile_simple_function() {
        let source = r#"
            def answer() -> int {
                return 42
            }
            answer()
        "#;
        // Just test that it compiles
        let mut program = parse(source).unwrap();
        typecheck(&mut program).unwrap();

        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test");
        codegen.compile(&program).unwrap();
    }

    #[test]
    fn test_compile_add() {
        let source = r#"
            def add(a: int, b: int) -> int {
                return a + b
            }
            add(3, 4)
        "#;
        let mut program = parse(source).unwrap();
        typecheck(&mut program).unwrap();

        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test");
        codegen.compile(&program).unwrap();
    }
}
