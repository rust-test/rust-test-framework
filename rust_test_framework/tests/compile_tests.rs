#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/setup_missing.rs");
    t.compile_fail("tests/ui/teardown_missing.rs");
    t.pass("tests/ui/fixture_success.rs");
    t.pass("tests/ui/qualified_paths.rs");
}
