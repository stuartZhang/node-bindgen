use std::time::Duration;
use std::io::Error as IoError;

use fluvio_future::timer::sleep;

use ohos_node_bindgen::derive::node_bindgen;


struct MyObject {
    val: f64,
}


#[ohos_node_bindgen]
impl MyObject {


    #[node_bindgen(constructor)]
    fn new(val: f64) -> Self {
        Self { val }
    }

    /// loop and emit event
    #[ohos_node_bindgen]
    async fn sleep<F: Fn(String)>(&self,cb: F)  {

        println!("sleeping");
        sleep(Duration::from_secs(1)).await;
        let msg = format!("hello world");
        cb(msg);

    }


}

fn main() {

}
