use crate::parser::{PestRule as Rule, ScriptParser};
use pest::Parser;

/// `ScriptTimezone` is not to be confused with
/// current time state; it really is an offset defined
/// by the user's input (script) and is set to either mirror the
/// user's choosen timezone by default or set a custom one.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ScriptTimezone {
    /// Assume the user wants to infer the script's timezone
    /// to be the same as their choosen timezone.
    Mirror,

    /// Set custom script timezone and recalculate
    /// any durations with this offset in mind.
    ///
    /// In seconds.
    Custom(i32),
}

pub struct ScriptTimezoneParser;

impl ScriptTimezoneParser {
    pub fn parse(timezonelike_expr: &str) -> i32 {
        let mut power = 1;
        let mut hours = 0;
        let mut minutes = 0;

        match ScriptParser::parse(Rule::TimezoneExpr, timezonelike_expr) {
            Ok(pairs) => {
                println!("{:#?}", pairs);

                for expr in pairs {
                    match expr.as_rule() {
                        Rule::TimezoneExpr => {
                            for prop in expr.into_inner() {
                                match prop.as_rule() {
                                    Rule::TimezoneNegative => power = -1,
                                    Rule::TimezoneHours1 | Rule::TimezoneHours2 => {
                                        hours = prop.as_str().parse().unwrap();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        }

        power * ((hours * 3600) + (minutes * 60))
    }
}
