use ohos_node_bindgen::derive::node_bindgen;



struct MyObject {
    val: f64,
}


#[ohos_node_bindgen]
impl MyObject {

    #[node_bindgen(xyz)]
    fn new(val: f64) -> Self {
        Self { val }
    }

}


fn main() {
    
}