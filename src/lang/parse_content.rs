use crate::error::parse_error::ParseError;

use super::munyo_parser::{Pairs, Rule};


pub(crate) fn parse_content(mut pairs: Pairs, starting_text: &str) -> Result<String, ParseError> {
    let mut s = String::with_capacity(8);
    s.push_str(starting_text);
    for pair in pairs {
        match pair.as_rule() {
            Rule::char_seq => {
                s.push_str(pair.as_str());
            }
            Rule::escaped => match pair.as_str() {
                r"\\" => {
                    s.push('\\');
                }
                r"\|" => {
                    s.push('|');
                }
                r"\n" => {
                    s.push('\n');
                }
                r"\r" => {
                    s.push('\r');
                }
                r"\t" => {
                    s.push('\t');
                }
                _ => {
                    unreachable!()
                }
            },
            _ => {
                unreachable!();
            }
        }
    }
    Ok(s)
}