use ohos_node_bindgen::derive::node_bindgen;
use ohos_node_bindgen::core::NjError;

#[node_bindgen()]
fn hello(count: i32) -> String {
    format!("hello world {count}")
}

#[ohos_node_bindgen]
fn sum(first: i32, second: i32) -> i32 {
    first + second
}

// throw error if first > second, otherwise return sum
#[ohos_node_bindgen]
fn min_max(first: i32, second: i32) -> Result<i32, NjError> {
    if first > second {
        Err(NjError::Other("first arg is greater".to_owned()))
    } else {
        Ok(first + second)
    }
}

#[node_bindgen(name = "multiply")]
fn mul(first: i32, second: i32) -> i32 {
    first * second
}

/// add second if supplied
#[node_bindgen()]
fn sum2(first: i32, second_arg: Option<i32>) -> i32 {
    if let Some(second) = second_arg {
        first + second
    } else {
        first
    }
}

#[node_bindgen()]
fn string(first: String, second_arg: Option<String>) -> String {
    if let Some(second) = second_arg {
        format!("{first} {second}")
    } else {
        first
    }
}

#[ohos_node_bindgen]
fn give_null(null: bool) -> Option<bool> {
    if null {
        None
    } else {
        Some(null)
    }
}

#[ohos_node_bindgen]
fn give_str(s: &str) -> String {
    s.to_string()
}
