mod utils;

const UTC_PLUS_3: i32 = 3 * 3600;

mod timezones {
    use super::*;

    test! {
        name: utc_variants_of_the_same,
        now: time! {04:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            short => "at 10am utc+4" match duration!{05:00},
            long_with_colon => "at 10am utc+04:00" match duration!{05:00},
            long => "at 10am utc+0400" match duration!{05:00},
        }
    }

    test! {
        name: minus_sign,
        now: time! {15:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            at_10am_utc_m_3 => "at 8pm utc-3" match duration!{11:00},
        }
    }

    test! {
        name: with_minutes,
        now: time! {15:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            at_10am_utc_m_3 => "at 8pm utc-0330" match duration!{11:30},
        }
    }

    test! {
        name: with_text,
        now: time! {12:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            before => "should start at 3pm utc+01:50" match duration!{04:10},
            after => "at 3pm utc+01:50 is when it starts" match duration!{04:10},
        }
    }

    test! {
        name: with_duration,
        now: time! {12:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            p10 => "at 3pm utc+01:50 10m" match duration!{04:20},
        }
    }

    test! {
        name: with_duration_and_text,
        now: time! {12:00 UTC_PLUS_3},
        offset: UTC_PLUS_3,
        variants: {
            t01 => "should start at 3pm utc+01:50 and I give myself additional 10m to prepare" match duration!{04:20},
        }
    }
}
