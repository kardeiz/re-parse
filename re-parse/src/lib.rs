extern crate regex;
extern crate serde_urlencoded;
extern crate url;
extern crate serde;
extern crate re_parse_macros;
extern crate lazy_static;

use url::form_urlencoded;

pub use re_parse_macros::*;

pub use serde_urlencoded::de::Error;
pub use regex::Regex;
pub use lazy_static::lazy_static;

pub fn with_pattern_from_str<T>(re: &regex::Regex, s: &str) -> Result<T, Error> 
    where T: serde::de::DeserializeOwned {

    let mut serializer = form_urlencoded::Serializer::new(String::new());

    if let Some(caps) = re.captures(s) {
        for opt_name in re.capture_names() {
            if let Some(name) = opt_name {
                if let Some(val) = caps.name(name) {
                    serializer.append_pair(name, val.as_str());
                }            
            }
        }
    }

    let encoded = serializer.finish();

    Ok(serde_urlencoded::from_str(&encoded)?)
}



#[cfg(test)]
mod tests {

    extern crate serde_derive;

    use self::serde_derive::*;
    use super::*;

    #[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize)]
    struct DateTime {
        year: u32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    }

    #[test]
    fn it_works() {
        let re = Regex::new(r"(?x)
            \[
                (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                \s+
                (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
            \]
        ").unwrap();

        let out = with_pattern_from_str::<DateTime>(&re, "[1518-11-01 00:00]");

        println!("{:?}", &out);

    }
}
