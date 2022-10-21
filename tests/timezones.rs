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
}
