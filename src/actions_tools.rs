pub fn set_output(name: &str, value: &str) {
    println!("::set-output name={}::{}", name, value);
}

pub fn group_lines(name: &str) {
    println!("::group::{}", name);
}

pub fn close_group() {
    println!("::endgroup::");
}
