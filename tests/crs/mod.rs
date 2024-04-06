use std::path::PathBuf;

use http::Request;
use rust_ftw::file::{input::Input, File};

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

pub fn requests_for_crs(version: &str) -> Vec<Result<Request<Vec<u8>>, (Input, http::Error)>> {
    tests_for_crs(version)
        .flat_map(|path| {
            let file = File::from_path(&path).unwrap();
            file.inputs()
                .map(|i| i.request().map_err(|e| (i.clone(), e)))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn unparsable_requests_for_crs(version: &str) -> Vec<(Input, http::Error)> {
    requests_for_crs(version)
        .into_iter()
        .filter_map(|r| r.err())
        .collect()
}
