#![cfg(test)]

type SmallString8 = smallstr::SmallString<[u8; 8]>;

#[test]
fn test_smallstr() {
    let str_new: SmallString8 = SmallString8::from("Hello");
    assert_eq!(str_new.as_str(), "Hello");
}

#[test]
fn test_smallstr_too_long() {
    let str_new: SmallString8 = SmallString8::from("Hello this is a very long string");
    assert_eq!(str_new.as_str(), "Hello this is a very long string");
}
