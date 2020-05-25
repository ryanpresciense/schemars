mod util;
use util::*;
use macaddr:MacAddr;

#[test]
fn macaddr() -> TestResult {
    test_default_generated_schema::<MacAddr>("macaddr")
}
