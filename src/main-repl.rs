use crate::slyther::ExprsParser;
use lalrpop_util::lalrpop_mod;
use linefeed::Interface;

lalrpop_mod!(pub slyther);

fn main() {
    let reader = Interface::new("boxr").unwrap();
    reader.set_prompt("==> ").unwrap();
    let parser = ExprsParser::new();
    loop {
        match reader.read_line().unwrap() {
            linefeed::ReadResult::Input(line) => {
                let expr = parser.parse(&line).unwrap();
                println!("{:#?}", expr);
            }
            linefeed::ReadResult::Eof => break,
            linefeed::ReadResult::Signal(_) => break,
        }
    }
}
