use ohos_node_bindgen::derive::node_bindgen;
use ohos_node_bindgen::sys::napi_value;
use ohos_node_bindgen::core::NjError;
use ohos_node_bindgen::core::val::JsEnv;
use ohos_node_bindgen::core::TryIntoJs;
use ohos_node_bindgen::core::val::JsObject;

use serde_json::value::Value;
use serde_json::map::Map;

// The recommended way of transforming to json
#[ohos_node_bindgen]
struct StandardJson {
    some_name: String,
    a_number: i64,
}

#[ohos_node_bindgen]
struct Outer {
    val: Inner,
}

#[ohos_node_bindgen]
struct Inner(String);

#[ohos_node_bindgen]
struct UnitStruct;

#[ohos_node_bindgen]
enum ErrorType {
    WithMessage(String, usize),
    WithFields { val: usize },
    UnitError,
}

#[ohos_node_bindgen]
struct WithSerdeJson {
    val: Value,
}

struct CustomJson {
    val: f64,
}

impl TryIntoJs for CustomJson {
    /// serialize into json object, with custom field names
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        // create JSON
        let mut json = JsObject::new(*js_env, js_env.create_object()?);

        let js_val = js_env.create_double(self.val)?;
        json.set_property("customFieldName", js_val)?;

        json.try_to_js(js_env)
    }
}

/// return json object
#[ohos_node_bindgen]
fn custom_json() -> CustomJson {
    CustomJson { val: 10.0 }
}

#[ohos_node_bindgen]
fn standard_json() -> StandardJson {
    StandardJson {
        some_name: "John".to_owned(),
        a_number: 1337,
    }
}

#[ohos_node_bindgen]
fn multilevel_json() -> Outer {
    Outer {
        val: Inner("hello".to_owned()),
    }
}

#[ohos_node_bindgen]
fn unit_struct() -> UnitStruct {
    UnitStruct
}

#[ohos_node_bindgen]
fn with_message() -> ErrorType {
    ErrorType::WithMessage("test".to_owned(), 321)
}

#[ohos_node_bindgen]
fn with_fields() -> ErrorType {
    ErrorType::WithFields { val: 123 }
}

#[ohos_node_bindgen]
fn with_unit() -> ErrorType {
    ErrorType::UnitError
}

#[ohos_node_bindgen]
fn failed_result_with_fields() -> Result<(), ErrorType> {
    Err(ErrorType::WithFields { val: 987 })
}

#[ohos_node_bindgen]
async fn async_result_failed_unit() -> Result<(), ErrorType> {
    Err(ErrorType::UnitError)
}

#[ohos_node_bindgen]
fn with_serde_json() -> WithSerdeJson {
    let mut map = Map::new();
    map.insert("first".to_owned(), Value::Bool(true));
    map.insert("second".to_owned(), Value::String("hello".to_owned()));

    WithSerdeJson {
        val: Value::Object(map),
    }
}
