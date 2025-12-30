#![allow(unused_imports, unused_variables)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use cfg_if::cfg_if;

use calculator::Compile;
cfg_if! {
    if #[cfg(feature = "jit")] {
        use calculator::Jit as Engine;
    }
    else if #[cfg(feature = "interpreter")] {
        use calculator::Interpreter as Engine;
    }
    else if #[cfg(feature = "vm")]{
        use calculator::vm::bytecode::Interpreter as Engine;
        use calculator::VM;
    }
}

// ANCHOR: repl
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Calculator prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                cfg_if! {
                    if #[cfg(any(feature = "jit", feature = "interpreter"))] {
                        match Engine::from_source(line) {
                            Ok(result) => println!("{}", result),
                            Err(e) => eprintln!("{}", e),
                        };
                    }
                    else if #[cfg(feature = "vm")] {
                        let byte_code = Engine::from_source(line);
                        println!("byte code: {:?}", byte_code);
                        let mut vm = VM::new(byte_code);
                        vm.run();
                        println!("{}", vm.pop_last());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
    // ANCHOR_END: repl
}
