use organization;

// separate non-test files by using old naming convention for module
// i.e. tests/common/mod.rs
mod common;


// each file in tests/ is a separate create, so they will each need to bring in external crates
// they need
// we do not need cfg(test) since Rust treats files in tests/ specially
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, organization::add_two(2));

}
