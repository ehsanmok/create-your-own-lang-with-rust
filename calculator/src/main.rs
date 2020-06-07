use cfg_if::cfg_if;

use calculator::Compile;
cfg_if! {
    if #[cfg(feature = "default")] {
        use calculator::Jit as Engine;
    }
    else {
        use calculator::Interpreter as Engine;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not input file was provided");
        std::process::exit(-1);
    }
    println!(
        "{:?}",
        Engine::from_source(&std::fs::read_to_string(&args[1]).unwrap()).unwrap()
    );
}
