use std::path::PathBuf;

use rust_ftw::file::File;

pub fn tests_for_crs(version: &str) -> impl Iterator<Item = PathBuf> {
    glob::glob(&format!(
        "tests/crs/{version}/tests/regression/tests/*/*.yaml"
    ))
    .unwrap()
    .map(Result::unwrap)
}

pub fn assert_tests_parse(version: &str) {
    for path in tests_for_crs(version) {
        if let Err(e) = File::from_path(&path) {
            panic!("failed to parse {:?}: {}", path, e);
        }
    }
}
