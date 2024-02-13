use ohos_node_bindgen::derive::node_bindgen;

 
/// no argument and no result
#[ohos_node_bindgen]
fn example1() {        
}


/// single argument with result
#[ohos_node_bindgen]
fn example2(arg1: i32) -> i32 {        
    arg1
}

/// multiple arguments
#[ohos_node_bindgen]
fn example3(_arg1: bool,_arg2: i32,_arg3: String) -> i32 {        
    4
}


#[node_bindgen(name="hello2")]
fn example4(count: i32) -> i32 {        
    count
}



fn main() {
    
}