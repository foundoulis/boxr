use boxr::slyther::ExprsParser;
use linefeed::Interface;

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
