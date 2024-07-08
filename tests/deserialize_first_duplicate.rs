use serde::Serialize;
use serde_deserialize_duplicates::DeserializeFirstDuplicate;

#[test]
fn test_one_value() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        pub value: String,
    }

    let data = r#"
        {
            "value": "first",
            "value": "second"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.value, "first".to_owned());
}

#[test]
fn test_multiple_values() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        pub a: String,
        pub b: String,
        pub c: String,
    }

    let data = r#"
        {
            "a": "first",
            "b": "second",
            "c": "third"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.a, "first".to_owned());
    assert_eq!(parsed_struct.b, "second".to_owned());
    assert_eq!(parsed_struct.c, "third".to_owned());
}

#[test]
fn test_renamed_value() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(rename = "other_name")]
        pub original_name: String,
    }

    let data = r#"
        {
            "original_name": "first",
            "other_name": "second"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.original_name, "first".to_owned());
}

#[test]
fn test_renamed_value_renamed_first() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(rename = "other_name")]
        pub original_name: String,
    }

    let data = r#"
        {
            "other_name": "first",
            "original_name": "second"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.original_name, "first".to_owned());
}

#[test]
fn test_renamed_value_only_renamed() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(rename = "other_name")]
        pub original_name: String,
    }

    let data = r#"
        {
            "other_name": "exists"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.original_name, "exists".to_owned());
}

#[test]
fn test_aliased_value() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(alias = "other_name")]
        pub original_name: String,
    }

    let data = r#"
        {
            "original_name": "first",
            "other_name": "second"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.original_name, "first".to_owned());
}

#[test]
fn test_aliased_value_aliased_first() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(alias = "other_name")]
        pub original_name: String,
    }

    let data = r#"
        {
            "other_name": "first",
            "original_name": "second"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.original_name, "first".to_owned());
}

#[test]
fn test_aliased_value_only_aliased() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(alias = "other_name")]
        pub original_name: String,
    }

    let data = r#"
        {
            "other_name": "exists"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.original_name, "exists".to_owned());
}

#[test]
fn test_multiple_aliases() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(alias = "b", alias = "c")]
        pub a: String,
    }

    let data = r#"
        {
            "a": "first",
            "b": "second",
            "c": "third"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.a, "first".to_owned());
}

#[test]
fn test_optional_value_some() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        pub value: Option<String>,
    }

    let data = r#"
        {
            "value": "exists"
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.value, Some("exists".to_owned()));
} 


#[test]
fn test_optional_value_none_default() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        #[serde(default)]
        pub value: Option<String>,
    }

    let data = r#"
        {
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.value, None);
}

#[test]
fn test_collection_value() {
    #[derive(Serialize, DeserializeFirstDuplicate)]
    struct TestStruct {
        pub value: Vec<String>,
    }

    let data = r#"
        {
            "value": ["a", "b"]
        }"#;

    let parsed_struct: TestStruct = serde_json::from_str(data).unwrap();

    assert_eq!(parsed_struct.value, vec!["a".to_owned(), "b".to_owned()]);
}
