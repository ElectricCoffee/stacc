use regex::Regex;

lazy_static!{
    pub static ref NUMBER: Regex      = Regex::new(r"(\d*\.\d+|\d+)").unwrap();
    pub static ref BOOLEAN: Regex     = Regex::new(r"(true|false)").unwrap();
    pub static ref SYMBOL: Regex      = Regex::new(r"[$a-zA-Z][$\w]*").unwrap();
    pub static ref BEGIN_SCOPE: Regex = Regex::new(r"\(").unwrap();
    pub static ref END_SCOPE: Regex   = Regex::new(r"\)").unwrap();
    pub static ref BEGIN_LIST: Regex  = Regex::new(r"\[").unwrap();
    pub static ref END_LIST: Regex    = Regex::new(r"\]").unwrap();
}