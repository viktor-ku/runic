#[macro_export]
macro_rules! time {
    ($h:literal:$m:literal-$s:literal) => {{
        use chrono::{TimeZone, Utc};

        let dt = Utc.ymd(2021, 1, 1).and_hms($h, $m, $s);

        dt.timestamp()
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
        #[cfg(not(target_arch = "wasm32"))]
        fn $name() {
            use runic::{Runic};
            use pretty_assertions::assert_eq;

            {
                println!("UTC0");

                let mut rune = Runic::new($describe);

                rune.timestamp($now)
                    .offset(0);

                let rune = rune.describe();

                assert_eq!(
                    rune.total(),
                    $total,
                    "rune total should match case total"
                );
            }
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
        (($h * 60 * 60) + ($m * 60) + $s) as u64
    };

    ($h:literal:$m:literal) => {
        crate::duration! {$h:$m-00}
    };

    (::$s:literal) => {
        crate::duration! {00:00-$s}
    };
}
