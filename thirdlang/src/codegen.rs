//! LLVM Code Generation for Thirdlang
//!
//! Extends Secondlang's codegen with:
//! - Class memory layout (fields as struct)
//! - Object allocation (malloc)
//! - Object deallocation (free + destructor)
//! - Method compilation (first param is self pointer)
//! - Field access/assignment via GEP
//! - LLVM New Pass Manager (NPM) for optimization

use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType};
use inkwell::values::{
    BasicMetadataValueEnum, BasicValueEnum, FunctionValue, IntValue, PointerValue,
};
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};

use crate::ast::{
    AssignTarget, BinaryOp, ClassDef, Expr, MethodDef, Program, Stmt, TopLevel, TypedExpr, UnaryOp,
};
use crate::types::{ClassRegistry, Type};

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
    /// Map from class names to their LLVM struct types
    class_types: HashMap<String, StructType<'ctx>>,
    /// Class registry from type checker
    classes: ClassRegistry,
    /// Current function being compiled
    current_fn: Option<FunctionValue<'ctx>>,
    /// Current class being compiled (for method compilation)
    current_class: Option<String>,
}
// ANCHOR_END: codegen_struct

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str, classes: ClassRegistry) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
            functions: HashMap::new(),
            class_types: HashMap::new(),
            classes,
            current_fn: None,
            current_class: None,
        }
    }

    // ANCHOR: compile
    /// Compile a program and return the module
    pub fn compile(&mut self, program: &Program) -> Result<(), String> {
        // Declare libc functions
        self.declare_libc_functions();

        // First pass: create LLVM struct types for classes
        for item in program {
            if let TopLevel::Class(class) = item {
                self.create_class_type(class)?;
            }
        }

        // Second pass: declare all functions and methods
        for item in program {
            match item {
                TopLevel::Class(class) => {
                    self.declare_class_methods(class)?;
                }
                TopLevel::Stmt(Stmt::Function {
                    name,
                    params,
                    return_type,
                    ..
                }) => {
                    self.declare_function(name, params, return_type)?;
                }
                _ => {}
            }
        }

        // Third pass: compile function and method bodies
        for item in program {
            match item {
                TopLevel::Class(class) => {
                    self.compile_class(class)?;
                }
                TopLevel::Stmt(stmt @ Stmt::Function { .. }) => {
                    self.compile_stmt(stmt)?;
                }
                _ => {}
            }
        }

        // Fourth pass: create __main wrapper for all top-level non-function statements
        self.compile_main_wrapper_all(program)?;

        // Verify module
        self.module
            .verify()
            .map_err(|e| format!("Module verification failed: {}", e.to_string()))?;

        Ok(())
    }
    // ANCHOR_END: compile

    /// Declare libc functions (malloc, free)
    fn declare_libc_functions(&mut self) {
        let i64_type = self.context.i64_type();
        let ptr_type = self.context.ptr_type(AddressSpace::default());

        // void* malloc(size_t size)
        let malloc_type = ptr_type.fn_type(&[i64_type.into()], false);
        self.module.add_function("malloc", malloc_type, None);

        // void free(void* ptr)
        let free_type = self.context.void_type().fn_type(&[ptr_type.into()], false);
        self.module.add_function("free", free_type, None);
    }

    // ANCHOR: create_class_type
    /// Create LLVM struct type for a class
    fn create_class_type(&mut self, class: &ClassDef) -> Result<StructType<'ctx>, String> {
        let class_info = self
            .classes
            .get(&class.name)
            .ok_or_else(|| format!("Class {} not found in registry", class.name))?;

        // Create field types in order
        let mut field_types: Vec<BasicTypeEnum> = Vec::new();
        for field_name in &class_info.field_order {
            let field_type = class_info.get_field(field_name).unwrap();
            let llvm_type = self.llvm_basic_type(field_type)?;
            field_types.push(llvm_type);
        }

        // Create named struct type
        let struct_type = self.context.opaque_struct_type(&class.name);
        struct_type.set_body(&field_types, false);

        self.class_types.insert(class.name.clone(), struct_type);
        Ok(struct_type)
    }
    // ANCHOR_END: create_class_type

    /// Declare methods for a class
    fn declare_class_methods(&mut self, class: &ClassDef) -> Result<(), String> {
        for method in &class.methods {
            self.declare_method(&class.name, method)?;
        }
        Ok(())
    }

    /// Declare a method (function with self pointer as first param)
    fn declare_method(
        &mut self,
        class_name: &str,
        method: &MethodDef,
    ) -> Result<FunctionValue<'ctx>, String> {
        let ptr_type = self.context.ptr_type(AddressSpace::default());

        // Self pointer is first parameter
        let mut param_types: Vec<BasicMetadataTypeEnum> = vec![ptr_type.into()];

        // Add other parameters
        // Class-typed parameters are passed as pointers
        for (_, param_type) in &method.params {
            let llvm_type: BasicMetadataTypeEnum = if param_type.is_class() {
                ptr_type.into()
            } else {
                self.llvm_type(param_type)?.into()
            };
            param_types.push(llvm_type);
        }

        // Return type - always use i64 for simplicity (bools are extended to i64)
        let ret_type = self.context.i64_type();

        let fn_type = ret_type.fn_type(&param_types, false);
        let fn_name = format!("{}__{}", class_name, method.name);
        let function = self.module.add_function(&fn_name, fn_type, None);

        // Set parameter names
        function.get_nth_param(0).unwrap().set_name("self");
        for (i, (param_name, _)) in method.params.iter().enumerate() {
            function
                .get_nth_param((i + 1) as u32)
                .unwrap()
                .set_name(param_name);
        }

        self.functions.insert(fn_name, function);
        Ok(function)
    }

    /// Compile a class (all its methods)
    fn compile_class(&mut self, class: &ClassDef) -> Result<(), String> {
        self.current_class = Some(class.name.clone());

        for method in &class.methods {
            self.compile_method(&class.name, method)?;
        }

        self.current_class = None;
        Ok(())
    }

    /// Compile a method body
    fn compile_method(&mut self, class_name: &str, method: &MethodDef) -> Result<(), String> {
        let fn_name = format!("{}__{}", class_name, method.name);
        let function = self
            .functions
            .get(&fn_name)
            .cloned()
            .ok_or_else(|| format!("Method {} not declared", fn_name))?;

        // Create entry block
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Save current function
        self.current_fn = Some(function);
        self.variables.clear();

        // Allocate 'self' parameter (pointer to object)
        let self_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
        let self_alloca = self.create_entry_block_alloca(
            &function,
            "self",
            &Type::Class(class_name.to_string()),
        )?;
        self.builder.build_store(self_alloca, self_ptr).unwrap();
        self.variables.insert("self".to_string(), self_alloca);

        // Allocate other parameters
        for (i, (param_name, param_type)) in method.params.iter().enumerate() {
            let param_value = function.get_nth_param((i + 1) as u32).unwrap();
            let alloca = self.create_entry_block_alloca(&function, param_name, param_type)?;
            self.builder.build_store(alloca, param_value).unwrap();
            self.variables.insert(param_name.clone(), alloca);
        }

        // Compile body
        let mut last_value = None;
        for body_stmt in &method.body {
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

        Ok(())
    }

    /// Create a __main wrapper function for all top-level non-function statements
    fn compile_main_wrapper_all(&mut self, program: &Program) -> Result<(), String> {
        // Collect all non-function top-level statements
        let stmts: Vec<&Stmt> = program
            .iter()
            .filter_map(|item| match item {
                TopLevel::Stmt(stmt) if !matches!(stmt, Stmt::Function { .. }) => Some(stmt),
                _ => None,
            })
            .collect();

        if stmts.is_empty() {
            return Ok(());
        }

        // Create __main function: fn() -> i64
        let ret_type = self.context.i64_type();
        let fn_type = ret_type.fn_type(&[], false);
        let function = self.module.add_function("__main", fn_type, None);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        self.current_fn = Some(function);
        self.variables.clear();

        // Compile all statements
        let mut last_value: Option<IntValue> = None;
        for stmt in stmts {
            last_value = self.compile_stmt(stmt)?;
        }

        // Return the last value (or 0 if no value)
        let ret_val = last_value.unwrap_or_else(|| self.context.i64_type().const_int(0, false));
        self.builder.build_return(Some(&ret_val)).unwrap();

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

        for (i, (param_name, _)) in params.iter().enumerate() {
            function
                .get_nth_param(i as u32)
                .unwrap()
                .set_name(param_name);
        }

        self.functions.insert(name.to_string(), function);
        Ok(function)
    }

    /// Get LLVM type for our type (returns IntType)
    fn llvm_type(&self, ty: &Type) -> Result<inkwell::types::IntType<'ctx>, String> {
        match ty {
            Type::Int => Ok(self.context.i64_type()),
            Type::Bool => Ok(self.context.bool_type()),
            Type::Unit => Ok(self.context.i64_type()),
            Type::Unknown => Ok(self.context.i64_type()),
            Type::Class(_) => Ok(self.context.i64_type()), // Pointers are represented as i64
            Type::Function { .. } | Type::Method { .. } => {
                Err("Cannot get LLVM type for function/method type".to_string())
            }
        }
    }

    /// Get LLVM basic type for our type (for struct fields)
    fn llvm_basic_type(&self, ty: &Type) -> Result<BasicTypeEnum<'ctx>, String> {
        match ty {
            Type::Int => Ok(self.context.i64_type().into()),
            Type::Bool => Ok(self.context.bool_type().into()),
            Type::Class(name) => {
                // Class fields that are themselves classes are stored as pointers
                let _struct_type = self
                    .class_types
                    .get(name)
                    .ok_or_else(|| format!("Unknown class type: {}", name))?;
                Ok(self.context.ptr_type(AddressSpace::default()).into())
            }
            _ => Ok(self.context.i64_type().into()),
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

                let entry = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry);

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
                let int_val = self.value_to_int(value)?;
                self.builder.build_return(Some(&int_val)).unwrap();
                Ok(Some(int_val))
            }

            Stmt::Assignment { target, value, .. } => {
                let val = self.compile_expr(value)?;

                match target {
                    AssignTarget::Var(name) => {
                        if let Some(ptr) = self.variables.get(name) {
                            self.builder.build_store(*ptr, val).unwrap();
                        } else {
                            let function = self.current_fn.unwrap();
                            let alloca =
                                self.create_entry_block_alloca(&function, name, &value.ty)?;
                            self.builder.build_store(alloca, val).unwrap();
                            self.variables.insert(name.clone(), alloca);
                        }
                    }
                    AssignTarget::Field { object, field } => {
                        let obj_val = self.compile_expr(object)?;
                        let obj_ptr = obj_val.into_pointer_value();

                        // Get field index
                        let class_name = object.ty.class_name().ok_or("Expected class type")?;
                        let class_info = self.classes.get(class_name).ok_or("Class not found")?;
                        let field_idx = class_info.field_index(field).ok_or("Field not found")?;

                        // Get struct type
                        let struct_type = self
                            .class_types
                            .get(class_name)
                            .ok_or("Class type not found")?;

                        // GEP to field
                        let field_ptr = self
                            .builder
                            .build_struct_gep(*struct_type, obj_ptr, field_idx as u32, "field_ptr")
                            .unwrap();

                        self.builder.build_store(field_ptr, val).unwrap();
                    }
                }

                Ok(Some(self.value_to_int(val)?))
            }

            // ANCHOR: compile_delete
            Stmt::Delete(expr) => {
                let obj_val = self.compile_expr(expr)?;
                let obj_ptr = obj_val.into_pointer_value();

                // Call destructor if exists
                if let Type::Class(class_name) = &expr.ty {
                    let class_info = self.classes.get(class_name);
                    if let Some(info) = class_info {
                        if info.has_destructor {
                            let dtor_name = format!("{}____del__", class_name);
                            if let Some(dtor) = self.functions.get(&dtor_name) {
                                self.builder
                                    .build_call(*dtor, &[obj_ptr.into()], "dtor")
                                    .unwrap();
                            }
                        }
                    }
                }

                // Call free
                let free_fn = self.module.get_function("free").unwrap();
                self.builder
                    .build_call(free_fn, &[obj_ptr.into()], "")
                    .unwrap();

                Ok(None)
            }
            // ANCHOR_END: compile_delete
            Stmt::Expr(expr) => {
                let val = self.compile_expr(expr)?;
                Ok(Some(self.value_to_int(val)?))
            }
        }
    }

    // ANCHOR: compile_expr
    /// Compile an expression
    fn compile_expr(&mut self, expr: &TypedExpr) -> Result<BasicValueEnum<'ctx>, String> {
        match &expr.expr {
            Expr::Int(n) => Ok(self.context.i64_type().const_int(*n as u64, false).into()),

            Expr::Bool(b) => Ok(self.context.bool_type().const_int(*b as u64, false).into()),

            Expr::Var(name) => {
                let ptr = self
                    .variables
                    .get(name)
                    .ok_or_else(|| format!("Undefined variable: {}", name))?;

                let load_type = if expr.ty.is_class() {
                    self.context
                        .ptr_type(AddressSpace::default())
                        .as_basic_type_enum()
                } else {
                    self.context.i64_type().as_basic_type_enum()
                };

                let val = self.builder.build_load(load_type, *ptr, name).unwrap();
                Ok(val)
            }

            // ANCHOR: compile_self_ref
            Expr::SelfRef => {
                let ptr = self.variables.get("self").ok_or("'self' not in scope")?;
                let val = self
                    .builder
                    .build_load(self.context.ptr_type(AddressSpace::default()), *ptr, "self")
                    .unwrap();
                Ok(val)
            }
            // ANCHOR_END: compile_self_ref
            Expr::Unary { op, expr: inner } => {
                let val = self.compile_expr(inner)?.into_int_value();
                let result = match op {
                    UnaryOp::Neg => self.builder.build_int_neg(val, "neg").unwrap(),
                    UnaryOp::Not => self.builder.build_not(val, "not").unwrap(),
                };
                Ok(result.into())
            }

            Expr::Binary { op, left, right } => {
                let l = self.compile_expr(left)?.into_int_value();
                let r = self.compile_expr(right)?.into_int_value();

                let result = match op {
                    BinaryOp::Add => self.builder.build_int_add(l, r, "add").unwrap(),
                    BinaryOp::Sub => self.builder.build_int_sub(l, r, "sub").unwrap(),
                    BinaryOp::Mul => self.builder.build_int_mul(l, r, "mul").unwrap(),
                    BinaryOp::Div => self.builder.build_int_signed_div(l, r, "div").unwrap(),
                    BinaryOp::Mod => self.builder.build_int_signed_rem(l, r, "mod").unwrap(),
                    BinaryOp::Lt => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SLT, l, r, "lt")
                            .unwrap();
                        self.builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap()
                    }
                    BinaryOp::Gt => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SGT, l, r, "gt")
                            .unwrap();
                        self.builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap()
                    }
                    BinaryOp::Le => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SLE, l, r, "le")
                            .unwrap();
                        self.builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap()
                    }
                    BinaryOp::Ge => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::SGE, l, r, "ge")
                            .unwrap();
                        self.builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap()
                    }
                    BinaryOp::Eq => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::EQ, l, r, "eq")
                            .unwrap();
                        self.builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap()
                    }
                    BinaryOp::Ne => {
                        let cmp = self
                            .builder
                            .build_int_compare(IntPredicate::NE, l, r, "ne")
                            .unwrap();
                        self.builder
                            .build_int_z_extend(cmp, self.context.i64_type(), "ext")
                            .unwrap()
                    }
                };
                Ok(result.into())
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
                Ok(call.try_as_basic_value().unwrap_basic())
            }

            // ANCHOR: compile_method_call
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let obj_val = self.compile_expr(object)?;
                let obj_ptr = obj_val.into_pointer_value();

                // Get class name
                let class_name = object.ty.class_name().ok_or("Expected class type")?;
                let fn_name = format!("{}__{}", class_name, method);

                let function = self
                    .functions
                    .get(&fn_name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined method: {}", fn_name))?;

                // Build argument list: self first, then other args
                let mut arg_values: Vec<BasicMetadataValueEnum> = vec![obj_ptr.into()];
                for arg in args {
                    arg_values.push(self.compile_expr(arg)?.into());
                }

                let call = self
                    .builder
                    .build_call(function, &arg_values, "call")
                    .unwrap();
                Ok(call.try_as_basic_value().unwrap_basic())
            }
            // ANCHOR_END: compile_method_call
            // ANCHOR: compile_field_access
            Expr::FieldAccess { object, field } => {
                let obj_val = self.compile_expr(object)?;
                let obj_ptr = obj_val.into_pointer_value();

                // Get field index
                let class_name = object.ty.class_name().ok_or("Expected class type")?;
                let class_info = self.classes.get(class_name).ok_or("Class not found")?;
                let field_idx = class_info.field_index(field).ok_or("Field not found")?;
                let field_type = class_info.get_field(field).ok_or("Field not found")?;

                // Get struct type
                let struct_type = self
                    .class_types
                    .get(class_name)
                    .ok_or("Class type not found")?;

                // GEP to field
                let field_ptr = self
                    .builder
                    .build_struct_gep(*struct_type, obj_ptr, field_idx as u32, "field_ptr")
                    .unwrap();

                // Load field value
                let load_type = self.llvm_basic_type(field_type)?;
                let val = self
                    .builder
                    .build_load(load_type, field_ptr, "field")
                    .unwrap();
                Ok(val)
            }
            // ANCHOR_END: compile_field_access
            // ANCHOR: compile_new
            Expr::New { class, args } => {
                // Get struct type and size
                let struct_type = self.class_types.get(class).ok_or("Class type not found")?;

                // Calculate size (number of fields * 8 bytes)
                let class_info = self.classes.get(class).ok_or("Class not found")?;
                let size = (class_info.size() * 8).max(8) as u64; // At least 8 bytes
                let size_val = self.context.i64_type().const_int(size, false);

                // Call malloc
                let malloc_fn = self.module.get_function("malloc").unwrap();
                let ptr = self
                    .builder
                    .build_call(malloc_fn, &[size_val.into()], "obj")
                    .unwrap()
                    .try_as_basic_value()
                    .unwrap_basic()
                    .into_pointer_value();

                // Initialize fields to zero
                for (i, _) in class_info.field_order.iter().enumerate() {
                    let field_ptr = self
                        .builder
                        .build_struct_gep(*struct_type, ptr, i as u32, "init_field")
                        .unwrap();
                    let zero = self.context.i64_type().const_int(0, false);
                    self.builder.build_store(field_ptr, zero).unwrap();
                }

                // Call constructor if exists
                let ctor_name = format!("{}____init__", class);
                if let Some(ctor) = self.functions.get(&ctor_name).cloned() {
                    let mut ctor_args: Vec<BasicMetadataValueEnum> = vec![ptr.into()];
                    for arg in args {
                        ctor_args.push(self.compile_expr(arg)?.into());
                    }
                    self.builder.build_call(ctor, &ctor_args, "").unwrap();
                }

                Ok(ptr.into())
            }
            // ANCHOR_END: compile_new
            Expr::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.compile_expr(cond)?.into_int_value();
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

                // Merge
                if then_has_terminator && else_has_terminator {
                    unsafe {
                        merge_bb.delete().unwrap();
                    }
                    Ok(self.context.i64_type().const_int(0, false).into())
                } else {
                    self.builder.position_at_end(merge_bb);
                    let phi = self
                        .builder
                        .build_phi(self.context.i64_type(), "phi")
                        .unwrap();

                    if !then_has_terminator {
                        phi.add_incoming(&[(&then_val, then_end)]);
                    }
                    if !else_has_terminator {
                        phi.add_incoming(&[(&else_val, else_end)]);
                    }

                    Ok(phi.as_basic_value())
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
                let cond_val = self.compile_expr(cond)?.into_int_value();
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
                Ok(self.context.i64_type().const_int(0, false).into())
            }

            Expr::Block(stmts) => {
                let mut last_val: BasicValueEnum =
                    self.context.i64_type().const_int(0, false).into();
                for stmt in stmts {
                    if let Some(v) = self.compile_stmt(stmt)? {
                        last_val = v.into();
                    }
                }
                Ok(last_val)
            }
        }
    }
    // ANCHOR_END: compile_expr

    /// Convert BasicValueEnum to IntValue
    fn value_to_int(&self, val: BasicValueEnum<'ctx>) -> Result<IntValue<'ctx>, String> {
        match val {
            BasicValueEnum::IntValue(i) => Ok(i),
            BasicValueEnum::PointerValue(p) => Ok(self
                .builder
                .build_ptr_to_int(p, self.context.i64_type(), "ptoi")
                .unwrap()),
            _ => Err("Cannot convert value to int".to_string()),
        }
    }

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

        let alloca_type: BasicTypeEnum = if ty.is_class() {
            self.context.ptr_type(AddressSpace::default()).into()
        } else {
            self.context.i64_type().into()
        };

        Ok(builder.build_alloca(alloca_type, name).unwrap())
    }

    /// Get the compiled module
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    /// Print LLVM IR to string
    pub fn print_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    // ANCHOR: run_passes
    /// Run LLVM optimization passes using the New Pass Manager
    ///
    /// # Arguments
    /// * `passes` - A comma-separated list of passes, e.g., "dce,mem2reg,instcombine"
    ///   or a preset like "default<O2>"
    ///
    /// # Common passes for teaching:
    /// - `dce` - Dead Code Elimination
    /// - `mem2reg` - Promote allocas to SSA registers
    /// - `instcombine` - Combine redundant instructions
    /// - `simplifycfg` - Simplify control flow graph
    /// - `gvn` - Global Value Numbering
    /// - `default<O0>` through `default<O3>` - Standard optimization levels
    pub fn run_passes(&self, passes: &str) -> Result<(), String> {
        // Initialize native target for the current machine
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| format!("Failed to initialize native target: {}", e))?;

        // Get the default target triple for this machine
        let triple = TargetMachine::get_default_triple();

        // Get the target from the triple
        let target = Target::from_triple(&triple)
            .map_err(|e| format!("Failed to get target from triple: {}", e))?;

        // Create target machine with default settings
        let target_machine = target
            .create_target_machine(
                &triple,
                "generic", // CPU
                "",        // Features
                OptimizationLevel::Default,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| "Failed to create target machine".to_string())?;

        // Create pass builder options
        let pass_options = PassBuilderOptions::create();
        pass_options.set_verify_each(true); // Verify IR after each pass

        // Run the passes
        self.module
            .run_passes(passes, &target_machine, pass_options)
            .map_err(|e| format!("Failed to run passes: {}", e))
    }
    // ANCHOR_END: run_passes
}

// ANCHOR: jit_run
/// JIT compile and run a program
pub fn jit_run(program: &Program, classes: ClassRegistry) -> Result<i64, String> {
    jit_run_with_opts(program, classes, None)
}
// ANCHOR_END: jit_run

// ANCHOR: jit_run_optimized
/// JIT compile and run a program with optional optimization passes
///
/// # Arguments
/// * `program` - The parsed and type-checked program
/// * `classes` - Class registry from type checking
/// * `passes` - Optional optimization passes (e.g., "dce,mem2reg,instcombine")
pub fn jit_run_with_opts(
    program: &Program,
    classes: ClassRegistry,
    passes: Option<&str>,
) -> Result<i64, String> {
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "thirdlang", classes);

    codegen.compile(program)?;

    // Run optimization passes if specified
    if let Some(pass_pipeline) = passes {
        codegen.run_passes(pass_pipeline)?;
    }

    // Create execution engine
    let engine = codegen
        .module
        .create_jit_execution_engine(OptimizationLevel::Default)
        .map_err(|e| format!("Failed to create JIT: {}", e.to_string()))?;

    // Call the __main wrapper function
    unsafe {
        let func: inkwell::execution_engine::JitFunction<unsafe extern "C" fn() -> i64> =
            engine.get_function("__main").map_err(|e| e.to_string())?;
        Ok(func.call())
    }
}
// ANCHOR_END: jit_run_optimized

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::typeck::typecheck;

    #[test]
    fn test_compile_class() {
        let source = r#"
            class Point {
                x: int
                y: int

                def __init__(self, x: int, y: int) {
                    self.x = x
                    self.y = y
                }

                def get_x(self) -> int {
                    return self.x
                }
            }
            p = new Point(10, 20)
            p.get_x()
        "#;
        let mut program = parse(source).unwrap();
        let classes = typecheck(&mut program).unwrap();

        let context = Context::create();
        let mut codegen = CodeGen::new(&context, "test", classes);
        codegen.compile(&program).unwrap();
    }
}
