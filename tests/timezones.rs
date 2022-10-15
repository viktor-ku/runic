mod utils;

const UTC_PLUS_3: i32 = 3 * 3600;

mod timezones {
    use super::*;

    test! {
        name: stuff,
        now: time! {04:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            // 10am UTC+3 in UTC is 7am
            // 10am UTC+4 in UTC is 6am
            // so if now is 4am +3 is 1am UTC
            //
            // so now is 1am UTC
            // our goal is 6am (10am +4 in UTC is 6)
            test => "at 10am +4" match duration!{05:00},
        }
    }
}
