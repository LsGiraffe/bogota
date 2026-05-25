use bogota::greeting;

#[test]
fn greeting_returns_hello_world() {
    assert_eq!(greeting(), "Hello, world!");
}
