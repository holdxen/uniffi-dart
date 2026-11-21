#[test]
fn v032_features() -> anyhow::Result<()> {
    uniffi_dart::testing::run_test("v032_features", "src/api.udl", None)
}
