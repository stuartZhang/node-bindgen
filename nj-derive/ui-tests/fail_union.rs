use ohos_node_bindgen::derive::node_bindgen;

#[ohos_node_bindgen]
union TestUnion {
    pub field1: u32,
    pub field2: f32
}