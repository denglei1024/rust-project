use clap::Parser;

fn main() {
    let args = Args::parse();
    let expression = args.expression;
    match meval::eval_str(expression) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error evaluating expression: {}", e),
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    expression: String
}