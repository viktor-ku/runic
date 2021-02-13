/// Everyone should have consistent results when testing
pub const UTC_OFFSET: i32 = 3 * 3_600;

#[macro_export]
macro_rules! time {
    ($h:literal:$m:literal-$s:literal) => {{
        use chrono::{FixedOffset, TimeZone};

        let dt = FixedOffset::east(utils::UTC_OFFSET)
            .ymd(2020, 9, 1)
            .and_hms($h, $m, $s);

        #[cfg(not(target_arch = "wasm32"))]
        {
            dt.timestamp() as u64
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Corresponds to js `Date.now()` which returns
            // number of milliseconds since Unix Epoch.
            dt.timestamp_millis() as f64
        }
    }};

    ($h:literal:$m:literal) => {
        crate::time! { $h: $m - 00 }
    };

    () => {
        crate::time! { 00: 00 - 00 }
    };
}

#[macro_export]
macro_rules! test {
    (
        name:$name:tt,
        now:$now:expr,
        total:$total:expr,
        describe:$describe:literal
    ) => {
        #[test]
        #[wasm_bindgen_test]
        fn $name() {
            let rune = runic::Runic::describe($describe, $now);
            pretty_assertions::assert_eq!(
                rune.total(),
                $total,
                "rune total should match case total"
            );
        }
    };

    (
        name:$name:tt,
        now:$now:expr,
        total:$total:expr,
        variants: {
            $($variant:tt => $describe:literal),*
            $(,)?
        }
    ) => {
        mod $name {
            use super::*;

            $(
                crate::test! {
                    name: $variant,
                    now: $now,
                    total: $total,
                    describe: $describe
                }
            )*
        }
    };

    (
        name:$name:tt,
        now:$now:expr,
        variants: {
            $($variant:tt => $describe:literal match $total:expr),*
            $(,)?
        }
    ) => {
        mod $name {
            use super::*;

            $(
                crate::test! {
                    name: $variant,
                    now: $now,
                    total: $total,
                    describe: $describe
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! duration {
    ($h:literal:$m:literal-$s:literal) => {
        (($h * 60 * 60 * 1_000) + ($m * 60 * 1_000) + ($s * 1_000)) as f64
    };

    ($h:literal:$m:literal) => {
        crate::duration! {$h:$m-00}
    };

    (::$s:literal) => {
        crate::duration! {00:00-$s}
    };
}
