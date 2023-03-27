use boxr::{evaluator::lisp_eval, logger, slyther::ExprsParser, types::scope::LexicalVarStorage};
use linefeed::Interface;

#[mutants::skip]
fn main() {
    logger::setup_logger(log::LevelFilter::Info).unwrap();

    let reader = Interface::new("boxr").unwrap();
    reader.set_prompt("==> ").unwrap();
    let parser = ExprsParser::new();
    loop {
        match reader.read_line().unwrap() {
            linefeed::ReadResult::Input(line) => {
                let exprs = parser.parse(&line).unwrap();
                for expr in exprs {
                    let result = Some(lisp_eval(&expr, LexicalVarStorage::new()));
                    match result {
                        Some(Ok(v)) => println!("{}", v),
                        Some(Err(e)) => log::error!("{:?}", e),
                        None => {}
                    }
                }
            }
            linefeed::ReadResult::Eof => break,
            linefeed::ReadResult::Signal(_) => break,
        }
    }
}
