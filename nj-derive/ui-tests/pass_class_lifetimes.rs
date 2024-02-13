
use ohos_node_bindgen::derive::node_bindgen;

struct Inner;

struct NamedScopeObject<'a>{
    val: &'a Option<Inner>,
}

#[ohos_node_bindgen]
impl NamedScopeObject<'_> {
    #[node_bindgen(constructor)]
    fn new() -> Self {
        Self { val: &None }
    }

}

fn main() {

}
