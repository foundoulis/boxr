use boxr::lexer::lex;
use linefeed::Interface;

fn main() {
    let reader = Interface::new("boxr").unwrap();
    reader.set_prompt("==> ").unwrap();
    loop {
        match reader.read_line().unwrap() {
            linefeed::ReadResult::Input(line) => {
                let lexer_raw = lex(line);
                println!("{:?}", lexer_raw);
            }
            linefeed::ReadResult::Eof => break,
            linefeed::ReadResult::Signal(_) => break,
        }
    }
}
