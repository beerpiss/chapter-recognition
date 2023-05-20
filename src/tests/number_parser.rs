use crate::get_chapter_number_from_match;

macro_rules! assert_number {
    ($number:expr, $expected:expr) => {
        assert_eq!(get_chapter_number_from_match($number), $expected)
    };
}

#[test]
fn normal_number() {
    assert_number!(("13", "9"), Some(13.9));
    assert_number!(("13", "10"), Some(13.1));
    assert_number!(("13", "005"), Some(13.005));
}

#[test]
fn alpha_lower_postfix() {
    assert_number!(("13", "a"), Some(13.1));
    assert_number!(("13", "j"), Some(13.0));
}

#[test]
fn alpha_upper_postfix() {
    assert_number!(("13", "A"), Some(13.1));
    assert_number!(("13", "J"), Some(13.0));
}

#[test]
fn unknown_case() {
    assert_number!(("13", "balls"), Some(13.2));
}

#[test]
fn special() {
    assert_number!(("13", "special"), Some(13.97));
}

#[test]
fn omake() {
    assert_number!(("13", "omake"), Some(13.98));
}

#[test]
fn extra() {
    assert_number!(("13", "extra"), Some(13.99));
}
