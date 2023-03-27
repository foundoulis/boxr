use std::fs;

use boxr::{evaluator::lisp_eval, slyther::ExprsParser, types::scope::LexicalVarStorage};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let file_name = args.file;
    let file: String = fs::read_to_string(&file_name).unwrap();

    let parser = ExprsParser::new();
    let exprs = parser.parse(&file).unwrap();

    for expr in exprs {
        println!("{:?}", expr.as_ref());
        println!("{:?}", lisp_eval(&expr, LexicalVarStorage::new()));
    }
}
