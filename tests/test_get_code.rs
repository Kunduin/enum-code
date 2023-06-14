#[derive(enum_code::Code)]
enum TestError {
    #[code(1)]
    Tuple(String),
    #[code(2)]
    #[allow(dead_code)]
    Struct { message: String },
    #[code(3)]
    Simple,
}

#[test]
fn test_code() {
    assert_eq!(TestError::Tuple("test".to_string()).get_code(), 1);
    assert_eq!(
        TestError::Struct {
            message: "test".to_string()
        }
        .get_code(),
        2
    );
    assert_eq!(TestError::Simple.get_code(), 3);
}
