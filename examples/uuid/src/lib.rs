use ohos_node_bindgen::derive::node_bindgen;

/// Create a new random Uuid to return
#[ohos_node_bindgen]
fn make_uuid() -> uuid::Uuid {
    uuid::Uuid::parse_str("f7509856-9ae5-4c07-976d-a5b3f983e4af").unwrap()
}

#[ohos_node_bindgen]
fn take_uuid(uuid: uuid::Uuid) {
    let string = uuid.to_string();
    assert_eq!(string, "f7509856-9ae5-4c07-976d-a5b3f983e4af");
}
