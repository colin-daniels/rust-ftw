mod crs;

#[test]
fn test_parse_crs410() {
    crs::assert_tests_parse("v4.1.0");
}

#[test]
fn test_req_crs410() {
    let unparsable = crs::unparsable_requests_for_crs("v4.1.0");
    assert_eq!(unparsable.len(), 39);
}

#[test]
fn test_parse_crs400() {
    crs::assert_tests_parse("v4.0.0");
}

#[test]
fn test_req_crs400() {
    let unparsable = crs::unparsable_requests_for_crs("v4.0.0");
    assert_eq!(unparsable.len(), 39);
}

#[test]
fn test_parse_crs335() {
    crs::assert_tests_parse("v3.3.5");
}

#[test]
fn test_req_crs335() {
    let unparsable = crs::unparsable_requests_for_crs("v3.3.5");
    assert_eq!(unparsable.len(), 13);
}

#[test]
fn test_parse_crs332() {
    crs::assert_tests_parse("v3.3.2");
}

#[test]
fn test_req_crs332() {
    let unparsable = crs::unparsable_requests_for_crs("v3.3.2");
    assert_eq!(unparsable.len(), 13);
}

#[test]
fn test_parse_crs323() {
    crs::assert_tests_parse("v3.2.3");
}

#[test]
fn test_req_crs323() {
    let unparsable = crs::unparsable_requests_for_crs("v3.2.3");
    assert_eq!(unparsable.len(), 0);
}
