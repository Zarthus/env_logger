#[test]
fn test_without_env_set() {
    zarthus_env_logger::init();
    log::info!("test_without_env_set");
}
