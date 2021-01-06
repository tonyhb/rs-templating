#[path = "lib.rs"]
mod rs_templating;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("error: template and args missing");
        std::process::exit(1);
    }

    match rs_templating::compile_and_execute(args[1].clone(), args[2].clone()) {
        Ok(str) => println!("{}", str),
        Err(e) => println!("error: {}", e),
    }
}
