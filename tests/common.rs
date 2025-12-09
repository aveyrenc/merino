use std::time::Duration;

use merino::*;

#[tokio::test]
/// Can we crate a new `Merino` instance
async fn merino_constructor() {
    assert!(Merino::new(
        1080,
        "127.0.0.1",
        Vec::new(),
        Vec::new(),
        Duration::from_millis(100)
    )
    .await
    .is_ok())
}
