use crate::parser::PestRule as Rule;
use pest::iterators::Pair;

#[derive(Debug, PartialEq)]
pub enum Part {
    None,
    Pm,
    Am,
}

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct At {
    pub hours: u32,
    pub minutes: u32,
    pub script_timezone: ScriptTimezone,
}

impl At {
    /// Converts combination of `hours` - `minutes` - `Am/Pm/None` to
    /// 24h format time in a form of `hours` - `minutes`.
    fn convert24h(hours: u32, minutes: u32, part: &Part) -> (u32, u32) {
        let mut hours = hours;

        match part {
            Part::Pm => {
                hours = match hours {
                    0 => 12,
                    1 => 13,
                    2 => 14,
                    3 => 15,
                    4 => 16,
                    5 => 17,
                    6 => 18,
                    7 => 19,
                    8 => 20,
                    9 => 21,
                    10 => 22,
                    11 => 23,
                    _ => hours,
                };
            }
            _ => {}
        }

        if hours == 24 {
            hours = 0;
        }

        if part == &Part::Am && hours == 12 {
            hours = 0;
        }

        (hours, minutes)
    }

    pub fn parse(expr: Pair<Rule>) -> Self {
        let mut hours = 0;
        let mut minutes = 0;
        let mut part = Part::None;
        let mut script_timezone = ScriptTimezone::Mirror;

        for prop in expr.into_inner() {
            match prop.as_rule() {
                Rule::Pm => part = Part::Pm,
                Rule::Am => part = Part::Am,
                Rule::AtHours => {
                    hours = prop
                        .as_str()
                        .parse()
                        .expect("could not parse {at hours} in script");
                }
                Rule::AtMinutes => {
                    minutes = prop
                        .as_str()
                        .parse()
                        .expect("could not parse {at minutes} in script");
                }
                Rule::Timezone => {
                    script_timezone = ScriptTimezone::Custom(
                        prop.as_str()
                            .parse::<i32>()
                            .expect("could not parse the timezone in script")
                            * 3600,
                    );
                }
                _ => {}
            }
        }

        let (hours, minutes) = Self::convert24h(hours, minutes, &part);

        Self {
            hours,
            minutes,
            script_timezone,
        }
    }
}

#[cfg(test)]
mod convert24h {
    use super::{At, Part};

    macro_rules! test {
        (
            $name:tt =>
            $actual_h:literal:$actual_m:literal $part:tt
            match
            $expected_h:literal:$expected_m:literal
        ) => {
            #[test]
            fn $name() {
                pretty_assertions::assert_eq!(
                    At::convert24h($actual_h, $actual_m, &Part::$part),
                    ($expected_h, $expected_m),
                );
            }
        };

        (
            $name:tt =>
            $actual_h:literal:$actual_m:literal
            match
            $expected_h:literal:$expected_m:literal
        ) => {
            test! { $name => $actual_h:$actual_m None match $expected_h:$expected_m }
        };
    }

    mod h12 {
        use super::*;

        mod am {
            use super::*;
            use pretty_assertions::assert_eq;

            test! { _00_00 => 00:00 Am match 00:00 }
            test! { _01_15 => 01:15 Am match 01:15 }
            test! { _02_30 => 02:30 Am match 02:30 }
            test! { _03_45 => 03:45 Am match 03:45 }
            test! { _04_00 => 04:00 Am match 04:00 }
            test! { _05_15 => 05:15 Am match 05:15 }
            test! { _06_30 => 06:30 Am match 06:30 }
            test! { _07_45 => 07:45 Am match 07:45 }
            test! { _08_00 => 08:00 Am match 08:00 }
            test! { _09_15 => 09:15 Am match 09:15 }
            test! { _10_30 => 10:30 Am match 10:30 }
            test! { _11_45 => 11:45 Am match 11:45 }
            test! { _12_00 => 12:00 Am match 00:00 }
        }

        mod pm {
            use super::*;
            use pretty_assertions::assert_eq;

            test! { _00_00 => 00:00 Pm match 12:00 }
            test! { _01_15 => 01:15 Pm match 13:15 }
            test! { _02_30 => 02:30 Pm match 14:30 }
            test! { _03_45 => 03:45 Pm match 15:45 }
            test! { _04_00 => 04:00 Pm match 16:00 }
            test! { _05_15 => 05:15 Pm match 17:15 }
            test! { _06_30 => 06:30 Pm match 18:30 }
            test! { _07_45 => 07:45 Pm match 19:45 }
            test! { _08_00 => 08:00 Pm match 20:00 }
            test! { _09_15 => 09:15 Pm match 21:15 }
            test! { _10_30 => 10:30 Pm match 22:30 }
            test! { _11_45 => 11:45 Pm match 23:45 }
            test! { _12_00 => 12:00 Pm match 12:00 }
        }

        // Weird cases, but we will support them for the user :3
        // Who knows, maybe user typed "AM"/"PM" after "19" by mistake?
        // In addition to that, I really don't want this function to ever fail!
        mod weird {
            use super::*;

            mod am {
                use super::*;
                use pretty_assertions::assert_eq;

                test! { _13_00 => 13:00 Am match 13:00 }
                test! { _14_15 => 14:15 Am match 14:15 }
                test! { _15_30 => 15:30 Am match 15:30 }
                test! { _16_45 => 16:45 Am match 16:45 }
                test! { _17_00 => 17:00 Am match 17:00 }
                test! { _18_15 => 18:15 Am match 18:15 }
                test! { _19_30 => 19:30 Am match 19:30 }
                test! { _20_45 => 20:45 Am match 20:45 }
                test! { _21_00 => 21:00 Am match 21:00 }
                test! { _22_15 => 22:15 Am match 22:15 }
                test! { _23_30 => 23:30 Am match 23:30 }
                test! { _24_00 => 24:00 Am match 00:00 }
            }

            mod pm {
                use super::*;
                use pretty_assertions::assert_eq;

                test! { _13_00 => 13:00 Pm match 13:00 }
                test! { _14_15 => 14:15 Pm match 14:15 }
                test! { _15_30 => 15:30 Pm match 15:30 }
                test! { _16_45 => 16:45 Pm match 16:45 }
                test! { _17_00 => 17:00 Pm match 17:00 }
                test! { _18_15 => 18:15 Pm match 18:15 }
                test! { _19_30 => 19:30 Pm match 19:30 }
                test! { _20_45 => 20:45 Pm match 20:45 }
                test! { _21_00 => 21:00 Pm match 21:00 }
                test! { _22_15 => 22:15 Pm match 22:15 }
                test! { _23_30 => 23:30 Pm match 23:30 }
                test! { _24_00 => 24:00 Pm match 00:00 }
            }
        }
    }

    mod h24 {
        use super::*;
        use pretty_assertions::assert_eq;

        test! { _00_00 => 00:00 match 00:00 }
        test! { _01_15 => 01:15 match 01:15 }
        test! { _02_30 => 02:30 match 02:30 }
        test! { _03_45 => 03:45 match 03:45 }
        test! { _04_00 => 04:00 match 04:00 }
        test! { _05_15 => 05:15 match 05:15 }
        test! { _06_30 => 06:30 match 06:30 }
        test! { _07_45 => 07:45 match 07:45 }
        test! { _08_00 => 08:00 match 08:00 }
        test! { _09_15 => 09:15 match 09:15 }
        test! { _10_30 => 10:30 match 10:30 }
        test! { _11_45 => 11:45 match 11:45 }
        test! { _12_00 => 12:00 match 12:00 }
        test! { _13_15 => 13:15 match 13:15 }
        test! { _14_30 => 14:30 match 14:30 }
        test! { _15_45 => 15:45 match 15:45 }
        test! { _16_00 => 16:00 match 16:00 }
        test! { _17_15 => 17:15 match 17:15 }
        test! { _18_30 => 18:30 match 18:30 }
        test! { _19_45 => 19:45 match 19:45 }
        test! { _20_00 => 20:00 match 20:00 }
        test! { _21_15 => 21:15 match 21:15 }
        test! { _22_30 => 22:30 match 22:30 }
        test! { _23_45 => 23:45 match 23:45 }
        test! { _24_00 => 24:00 match 00:00 }
    }
}
