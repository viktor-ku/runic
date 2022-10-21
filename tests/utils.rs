use chrono::{DateTime, FixedOffset, NaiveDate};

pub(crate) fn compute_timestamp(
    year: i32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    offset: i32,
) -> i64 {
    let naive = NaiveDate::from_ymd(year, month, day).and_hms(hours, minutes, seconds);

    let offset = FixedOffset::east(offset);

    let input = format!("{:?}{:?}", naive, offset);
    let dt = DateTime::parse_from_rfc3339(&input).unwrap();
    println!("dt {:#?}", dt);

    dt.timestamp()
}

#[macro_export]
macro_rules! time {
    ($h:literal : $m:literal - $s:literal $utc:expr) => {
        utils::compute_timestamp(2021, 1, 1, $h, $m, $s, $utc)
    };

    ($h:literal:$m:literal-$s:literal) => {
        crate::time! { $h: $m - $s 0 }
    };

    ($h:literal:$m:literal) => {
        crate::time! { $h:$m-00 }
    };

    ($h:literal:$m:literal $utc:expr) => {
        crate::time! { $h:$m-00 $utc }
    };

    () => {
        crate::time! { 00:00 }
    };
}

#[macro_export]
macro_rules! duration {
    ($h:literal:$m:literal-$s:literal) => {{
        (($h * 60 * 60) + ($m * 60) + $s) as u64
    }};

    ($h:literal:$m:literal) => {
        crate::duration! {$h:$m-00}
    };

    (::$s:literal) => {
        crate::duration! {00:00-$s}
    };
}

#[macro_export]
macro_rules! test {
    (
        name:$name:tt,
        now:$now:expr,
        total:$total:expr,
        describe:$describe:literal,
        offset:$offset:expr
    ) => {
        #[test]
        fn $name() {
            let runic = runic::Runic {
                script: $describe,
                timestamp: Some($now),
                offset: Some($offset),
            };

            pretty_assertions::assert_eq!(
                runic.describe().unwrap(),
                $total,
                "rune total should match case total"
            );

        }
    };

    (
        name:$name:tt,
        now:$now:expr,
        total:$total:expr,
        describe:$describe:literal
    ) => {
        crate::test! {
            name: $name,
            now: $now,
            total: $total,
            describe: $describe,
            offset: 0
        }
    };

    (
        name:$name:tt,
        now:$now:expr,
        total:$total:expr,
        offset:$offset:expr,
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
                    describe: $describe,
                    offset: $offset
                }
            )*
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
                    describe: $describe,
                    offset: 0
                }
            )*
        }
    };

    (
        name:$name:tt,
        now:$now:expr,
        offset:$offset:expr,
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
                    describe: $describe,
                    offset: $offset
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
                    now: $now, total: $total,
                    describe: $describe,
                    offset: 0
                }
            )*
        }
    };
}
