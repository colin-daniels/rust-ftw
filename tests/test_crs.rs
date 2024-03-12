use rust_ftw::file::File;

mod crs;

#[test]
fn test_parse_crs400() {
    crs::assert_tests_parse("v4.0.0");
}

#[test]
fn test_parse_crs335() {
    crs::assert_tests_parse("v3.3.5");
}

#[test]
fn test_parse_crs332() {
    crs::assert_tests_parse("v3.3.2");
}

#[test]
fn test_parse_crs323() {
    crs::assert_tests_parse("v3.2.3");
}

#[test]
fn test_req_crs335() {
    let mut failed = Vec::new();
    for path in crs::tests_for_crs("v3.3.5") {
        let file = File::from_path(&path).unwrap();

        for i in file.inputs().cloned() {
            if let Err(e) = i.request() {
                failed.push((i, e));
            }
        }
    }

    assert!(failed.is_empty(), "{:?}", failed);
}
