use js_protocol::runtime;

#[test]
fn test_call_argument_builder() {
    let arg = runtime::CallArgument::builder()
        .object_id("12345")
        .build();
    
    assert_eq!(arg.object_id(), Some(&std::borrow::Cow::Borrowed("12345")));
    
    let serialized = serde_json::to_string(&arg).unwrap();
    assert_eq!(serialized, r#"{"objectId":"12345"}"#);
}

#[test]
fn test_execution_context_description_builder() {
    let desc = runtime::ExecutionContextDescription::builder(
        42,
        "http://example.com",
        "my-context",
        "unique-id-123"
    ).build();
    
    assert_eq!(*desc.id(), 42);
    assert_eq!(desc.origin(), "http://example.com");
    assert_eq!(desc.name(), "my-context");
    assert_eq!(desc.unique_id(), "unique-id-123");
    
    let serialized = serde_json::to_string(&desc).unwrap();
    assert!(serialized.contains(r#""id":42"#));
    assert!(serialized.contains(r#""origin":"http://example.com""#));
}
