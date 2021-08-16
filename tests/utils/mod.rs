use std::env;

use chrono::{DateTime, TimeZone, Utc};
use std::io::Write;
use tempfile::{Builder, NamedTempFile};

pub fn ymd_midnight(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    Utc.ymd(year, month, day).and_hms(0, 0, 0)
}

pub fn write_tmp_file(file_content: &str) -> NamedTempFile {
    let mut temp_file = Builder::new()
        .prefix("pr-bump-test-")
        .suffix(".json")
        .tempfile()
        .unwrap();

    write!(temp_file.as_file_mut(), "{}", file_content).unwrap();

    temp_file
}

pub struct TestEnvVar<'a> {
    key: &'a str,
}

impl<'a> TestEnvVar<'a> {
    pub fn new(key: &'a str, value: &str) -> Self {
        env::set_var(key, value);

        TestEnvVar { key }
    }
}

impl Drop for TestEnvVar<'_> {
    fn drop(&mut self) {
        env::remove_var(self.key);
    }
}
