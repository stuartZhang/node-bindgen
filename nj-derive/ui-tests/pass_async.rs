use ohos_node_bindgen::derive::node_bindgen;


/// async callback
#[ohos_node_bindgen]
async fn example5<F: Fn(f64,String)>( seconds: i32, cb: F) {
}


#[ohos_node_bindgen]
async fn example6(arg: f64) -> f64 {
    0.0
}



fn main() {
}