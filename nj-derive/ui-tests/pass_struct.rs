use ohos_node_bindgen::derive::node_bindgen;

#[ohos_node_bindgen]
struct Something {
    pub field: usize
}

#[ohos_node_bindgen]
pub(crate) struct WithVisibility {
    pub field: usize
}

#[ohos_node_bindgen]
struct Lifetime<'a> {
    pub field: &'a usize
}

#[ohos_node_bindgen]
struct BoundGeneric<T>
    where T: Sync + std::fmt::Debug + ohos_node_bindgen::core::TryIntoJs
{
    pub field: T
}

#[ohos_node_bindgen]
struct BoundAndLifetimes<'a, T: Sync + std::fmt::Debug + ohos_node_bindgen::core::TryIntoJs + Clone> {
    pub field: &'a T
}

#[ohos_node_bindgen]
struct Simple {
    pub a_string: String,
    pub a_number: i64,
    pub a_float : f64
}

#[ohos_node_bindgen]
struct Unnamed(String, f64);

fn main() {
}