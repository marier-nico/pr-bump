use log::info;

pub fn set_output(name: &str, value: &str) {
    info!("::set-output name={}::{}", name, value);
}

pub fn group_lines(name: &str) {
    info!("::group::{}", name);
}

pub fn close_group() {
    info!("::endgroup::");
}
