use boxr::{
    evaluator::lisp_eval, logger, slyther::SExpressionsParser, types::scope::LexicalVarStorage,
};
use linefeed::Interface;

#[mutants::skip]
fn main() {
    logger::setup_logger(log::LevelFilter::Debug).unwrap();

    let reader = Interface::new("boxr").unwrap();
    reader.set_prompt("==> ").unwrap();
    let parser = SExpressionsParser::new();
    let mut global_stg = LexicalVarStorage::new();
    loop {
        match reader.read_line().unwrap() {
            linefeed::ReadResult::Input(line) => match parser.parse(&line) {
                Ok(exprs) => {
                    for expr in exprs {
                        let result = lisp_eval(&expr, &mut global_stg);
                        match result {
                            Ok(v) => println!("{}", v),
                            Err(e) => log::error!("{:?}", e),
                        }
                    }
                }
                Err(e) => log::error!("{:?}", e),
            },
            linefeed::ReadResult::Eof => break,
            linefeed::ReadResult::Signal(_) => break,
        }
    }
}
