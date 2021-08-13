use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all)]
    parser,
    "/src/command.rs"
);

pub(crate) fn parse<'input>(
    s: &'input str,
) -> Result<Command, ParseError<usize, Token<'input>, &'static str>> {
    // Lalrpop can't handle the emtpy string
    if s.trim().is_empty() {
        Ok(Command::Prev)
    } else {
        parser::CommandParser::new().parse(s)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Instr,
    Quit,
    Prev,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn t(s: &str, c: Command) {
        assert_eq!(parse(s).unwrap(), c);
    }

    // TODO: Test errors
    #[test]
    fn no_data() {
        t("i", Command::Instr);
        t(" i", Command::Instr);
        t("instr", Command::Instr);
        t("instruction", Command::Instr);

        t("q", Command::Quit);
        t("q   ", Command::Quit);
        t("quit", Command::Quit);
        t("  quit ", Command::Quit);

        t(" ", Command::Prev);
        t("", Command::Prev);
    }
}
