use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;

#[test]
fn test_one_value() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        pub value: String
    }

    // Mock data with duplicate values
    let data = r#"
        {
            "value": "first",
            "value": "second"
        }"#;

    // Deserialize our data into a ValueHolder, taking the first value we find
    let holder: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(holder.value, "first".to_owned());
}