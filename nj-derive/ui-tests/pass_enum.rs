use ohos_node_bindgen::derive::node_bindgen;
use ohos_node_bindgen::core::TryIntoJs;

#[ohos_node_bindgen]
enum TestEnum {
    Something(usize),
    Else {
        val: String
    },
    UnitVariant
}

#[ohos_node_bindgen]
enum Generic<T: TryIntoJs> {
    Container(T)
}

fn main() {
}