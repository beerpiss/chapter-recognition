macro_rules! assert_volume {
    ($title:expr, $chapter:expr, $expected:expr) => {
        assert_eq!(crate::parse_volume_number($title, $chapter), $expected)
    };
}

#[test]
fn basic_vol_prefix() {
    assert_volume!(
        "Mokushiroku Alice",
        "Mokushiroku Alice Vol.1 Ch. 4: Misrepresentation",
        1.0
    );

    assert_volume!("Solanin", "Solanin 028 Vol. 2", 2.0);
}

#[test]
fn name_containing_one_number() {
    let manga_title = "Bleach";

    assert_volume!(manga_title, "Bleach 567 Down With Snowwhite", -1.0);
}

#[test]
fn extreme_case() {
    let manga_title = "Onepunch-Man";

    assert_volume!(manga_title, "Onepunch-Man Punch Ver002 028", -1.0);
}

#[test]
fn chapter_containing_dot_v2() {
    assert_volume!("random", "Vol.1 Ch.5v.2: Alones", 1.0);
}

#[test]
fn volume_containing_multiple_zeros() {
    assert_volume!("random", "Vol.001 Ch.003: Kaguya Doesn't Know Much", 1.0);
}

#[test]
fn volume_short_form() {
    assert_volume!("New Normal", "New Normal v003 c002", 3.0);
}

#[test]
fn chapter_with_version_before_number() {
    assert_volume!(
        "Onepunch-Man",
        "Onepunch-Man Punch Ver002 086 : Creeping Darkness [3]",
        -1.0
    );
}

#[test]
fn version_attached_to_chapter_number() {
    assert_volume!(
        "Ansatsu Kyoushitsu",
        "Ansatsu Kyoushitsu 011v002: Assembly Time",
        -1.0
    );
}

#[test]
fn unparseable_volume() {
    assert_volume!("random", "Foo", -1.0);
    assert_volume!("random", "Ep.1 - Silence", -1.0);
    assert_volume!("random", "Episode 1", -1.0);
    assert_volume!("random", "Epilogue (1/3)", -1.0);
}

#[test]
fn season_as_volume() {
    assert_volume!("D.I.C.E", "D.I.C.E[Season 001] Ep. 007", 1.0);
    assert_volume!("", "[Season 1] Chapter 1: Nice", 1.0);
    assert_volume!("", "[Season 1] Ep. 75 - code (23)", 1.0);
}

#[test]
fn volume_in_format_sx() {
    assert_volume!("random", "(S1) Chapter 3 - The Sin", 1.0);
    assert_volume!("The Gamer", "S3 - Chapter 20", 3.0);
    assert_volume!("", "S1 E1 - Begin", 1.0);
}
