extern crate re_parse;
extern crate serde_derive;

use re_parse::*;
use serde_derive::Deserialize;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, ReParse)]
#[re_parse(regex=r#"(?x)
    \[
        (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
        \s+
        (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
    \]
"#)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

fn main() {

    let x: DateTime = "[1518-11-01 00:00]".parse().unwrap();

    println!("{:?}", &x);
}