use crate::parser::{Parser, ParseError};

pub fn repl() -> Result<(), rustyline::error::ReadlineError> {
    let mut rl = rustyline::DefaultEditor::new()?;
    while let Ok(line) = rl.readline(">> ") {
        match Parser::parse(&line) {
            Ok(ast) => {
                rl.add_history_entry(line)?;
                dbg!(ast);
            }
            Err(ParseError::Invalid(msg)) => {
                eprintln!("{}", msg);
            }
        }
    }
    Ok(())
}
