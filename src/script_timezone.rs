use crate::parser::PestRule as Rule;
use pest::iterators::Pair;

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
    pub fn parse(expr: Pair<Rule>) -> i32 {
        let mut power = 1;
        let mut hours: Option<i32> = None;
        let mut minutes = 0;

        for timezone_kind in expr.into_inner() {
            match timezone_kind.as_rule() {
                Rule::TimezoneLiteral => {
                    for prop in timezone_kind.into_inner() {
                        match prop.as_rule() {
                            Rule::TimezoneNegative => {
                                power = -1;
                            }
                            Rule::TimezoneHours => {
                                hours = Some(
                                    prop.as_str()
                                        .parse()
                                        .expect("could not parse script timezone hours"),
                                );
                            }
                            Rule::TimezoneHoursSimple => {
                                hours = Some(
                                    prop.as_str()
                                        .parse()
                                        .expect("could not parse script timezone hours"),
                                );
                            }
                            Rule::TimezoneMinutes => {
                                minutes = prop
                                    .as_str()
                                    .parse()
                                    .expect("could not parse script timezone minutes");
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    panic!(
                        "does not makes sense to access non-timezone props in a timezone pattern"
                    )
                }
            }
        }

        if let Some(hours) = hours {
            power * ((hours * 3600) + (minutes * 60))
        } else {
            panic!("specifying hours part in the literal timezone is required")
        }
    }
}
