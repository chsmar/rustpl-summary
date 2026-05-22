use rustpl_summary::ch11::add; // but the default crate name (lib.rs) is "rustpl-summary" from package name.

#[test]
fn ch11_it_adds_two() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
