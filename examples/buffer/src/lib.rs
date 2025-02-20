use serde::Serialize;

use ohos_node_bindgen::derive::node_bindgen;
use ohos_node_bindgen::core::buffer::{ArrayBuffer, JSArrayBuffer};
use ohos_node_bindgen::core::NjError;

#[derive(Serialize)]
struct MyStruct {
    a: String,
    b: i32,
}

/// byte array buffer from json bytes
#[ohos_node_bindgen]
fn test(b: i32) -> Result<ArrayBuffer, NjError> {
    let my_struct = MyStruct {
        a: "b".to_string(),
        b,
    };

    let json_string = serde_json::to_vec(&my_struct)
        .map_err(|err| NjError::Other(format!("serialization error: {err}")))?;

    Ok(ArrayBuffer::new(json_string))
}

use ohos_node_bindgen::core::val::JsEnv;
use ohos_node_bindgen::core::TryIntoJs;
use ohos_node_bindgen::core::val::JsObject;
use ohos_node_bindgen::sys::napi_value;

struct Record {
    buffer: ArrayBuffer,
    comment: String,
}

impl TryIntoJs for Record {
    /// serialize into json object
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        // create JSON
        let mut json = JsObject::create(js_env)?;

        json.set_property("buffer", self.buffer.try_to_js(js_env)?)?;
        json.set_property("comment", js_env.create_string_utf8(&self.comment)?)?;

        json.try_to_js(js_env)
    }
}

/// create byte array and wrap in side another json obj
#[ohos_node_bindgen]
fn test2(b: i32) -> Result<Record, NjError> {
    let my_struct = MyStruct {
        a: "b".to_string(),
        b,
    };

    let json_string = serde_json::to_vec(&my_struct)
        .map_err(|err| NjError::Other(format!("serialization error: {err}")))?;

    Ok(Record {
        buffer: ArrayBuffer::new(json_string),
        comment: "array buffer is cool!".to_owned(),
    })
}

#[ohos_node_bindgen]
fn test3(data: JSArrayBuffer) -> Result<String, NjError> {
    let message = String::from_utf8(data.to_vec())?;
    Ok(format!("reply {message}"))
}

#[ohos_node_bindgen]
fn test4(first: JSArrayBuffer, second: JSArrayBuffer) -> Result<String, NjError> {
    let message1 = String::from_utf8(first.to_vec())?;
    let message2 = String::from_utf8(second.to_vec())?;

    Ok(format!("{message1} {message2}"))
}
