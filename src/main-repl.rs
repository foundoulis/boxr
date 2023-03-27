use boxr::{evaluator::lisp_eval, slyther::ExprsParser, types::scope::LexicalVarStorage};
use linefeed::Interface;

#[mutants::skip]
fn main() {
    let reader = Interface::new("boxr").unwrap();
    reader.set_prompt("==> ").unwrap();
    let parser = ExprsParser::new();
    loop {
        match reader.read_line().unwrap() {
            linefeed::ReadResult::Input(line) => {
                let exprs = parser.parse(&line).unwrap();
                let mut result = None;
                for expr in exprs {
                    result = Some(lisp_eval(&expr, LexicalVarStorage::new()));
                }
                println!("Final result: {:?}", result);
            }
            linefeed::ReadResult::Eof => break,
            linefeed::ReadResult::Signal(_) => break,
        }
    }
}
