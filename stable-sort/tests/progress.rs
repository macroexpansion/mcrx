#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse-enum.rs");
    t.compile_fail("tests/02-not-enum-or-struct.rs");
    t.compile_fail("tests/03-out-of-order.rs");
    t.compile_fail("tests/04-variants-with-data.rs");
    t.pass("tests/05-brace-struct.rs");
    t.compile_fail("tests/06-not-brace-struct.rs");
    // t.compile_fail("tests/06-pattern-path.rs");
    // t.compile_fail("tests/07-unrecognized-pattern.rs");
    // t.pass("tests/08-underscore.rs");
}
