#[test]
fn test_with_env_set() {
    std::env::set_var("RUST_LOG", "foo");

    zarthus_env_logger::init();
    log::info!("test_with_env_set");

    std::env::remove_var("RUST_LOG");
}
