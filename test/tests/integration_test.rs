use rea_rs_test::{build_integration_test, ReaperVersion};
#[test]
fn main() {
    build_integration_test(ReaperVersion::latest());
}
