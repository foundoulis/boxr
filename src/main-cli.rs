use boxr::{evaluator::lisp_eval, logger, slyther::ExprsParser, types::scope::LexicalVarStorage};
use clap::Parser;
use log::LevelFilter;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

#[mutants::skip]
fn main() {
    logger::setup_logger(LevelFilter::Info).unwrap();

    let args = Args::parse();

    let file_name = args.file;
    let file: String = fs::read_to_string(&file_name).unwrap();

    let mut global_stg = LexicalVarStorage::new();
    let parser = ExprsParser::new();
    let exprs = parser.parse(&file).unwrap();

    for expr in exprs {
        log::debug!("{:?}", expr.as_ref());
        match lisp_eval(&expr, &mut global_stg) {
            Ok(_) => {}
            Err(e) => log::error!("{:?}", e),
        };
    }
}
