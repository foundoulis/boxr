use std::path::Path;

use boxr::errors::EvaluatorError;
use boxr::types::Cons;
use boxr::{
    evaluator::lisp_eval, logger, slyther::SExpressionsParser, types::scope::LexicalVarStorage,
};
use clap::ArgAction;
use lazy_static::lazy_static;
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::Repl;

lazy_static! {
    static ref PARSER: SExpressionsParser = SExpressionsParser::new();
}

struct BoxrContext(LexicalVarStorage);

#[mutants::skip]
fn eval(args: ArgMatches, context: &mut BoxrContext) -> reedline_repl_rs::Result<Option<String>> {
    let body: String = args
        .get_many("body")
        .unwrap()
        .into_iter()
        .map(|s: &String| format!(" {}", s.to_string()))
        .collect();
    let ast = PARSER.parse(body.as_str()).unwrap();
    let result = ast
        .iter()
        .map(|s| lisp_eval(s, &mut context.0))
        .collect::<Result<Vec<Cons>, EvaluatorError>>();
    Ok(Some(format!("{}", result.unwrap().iter().last().unwrap())))
}

#[mutants::skip]
fn lex(args: ArgMatches, _context: &mut BoxrContext) -> reedline_repl_rs::Result<Option<String>> {
    let body: String = args
        .get_many("body")
        .unwrap()
        .into_iter()
        .map(|s: &String| format!(" {}", s.to_string()))
        .collect();
    let ast = PARSER.parse(body.as_str()).unwrap();
    Ok(Some(format!("{:?}", ast)))
}

#[mutants::skip]
fn main() -> reedline_repl_rs::Result<()> {
    logger::setup_logger(log::LevelFilter::Debug).unwrap();

    let mut repl = Repl::new(BoxrContext(LexicalVarStorage::new()))
        .with_name("Boxr")
        .with_version("v0.1.0")
        .with_description("A Lisp interpreter written in Rust")
        .with_prompt("==")
        .with_history(Path::new(".boxr_history").to_path_buf(), 1000)
        .with_stop_on_ctrl_c(true)
        .with_stop_on_ctrl_d(true)
        .with_command(
            Command::new("eval").arg(Arg::new("body").action(ArgAction::Append)),
            eval,
        )
        .with_command(
            Command::new("lex").arg(Arg::new("body").action(ArgAction::Append)),
            lex,
        );
    repl.run()
}
