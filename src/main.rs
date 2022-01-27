use std::iter::FromIterator;

#[path = "rs_templating.rs"]
mod rs_templating;

// some binary that exists while webassembly interface types don't.
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("error: template and args missing");
        std::process::exit(1);
    }

    // rs-templating -vars "template data here"
    if args[1].eq_ignore_ascii_case("-vars") {
        let tpl = rs_templating::Template::init(args[2].clone()).unwrap_or_else(|_| 
            // use an empty template by default
            rs_templating::Template::init("".into()).unwrap()
        );

        let vars = tpl.get_variables();
        let val = serde_json::Value::from_iter(vars);
        println!("{}", val);
        return
    }

    // rs-templating "template data here" '{"key":"value"}'
    match rs_templating::compile_and_execute(args[1].clone(), args[2].clone()) {
        Ok(str) => println!("{}", str),
        Err(e) => println!("error: {}", e),
    }
}
