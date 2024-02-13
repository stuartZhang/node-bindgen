use quote::quote;
use proc_macro2::TokenStream;
use syn::Ident;
use syn::ItemFn;

use crate::util::ident;
use super::FnGeneratorCtx;
use super::generate_rust_invocation;

/// generate native code to be invoked by napi
pub fn generate_napi_code(ctx: &FnGeneratorCtx, input_fn: &ItemFn) -> TokenStream {
    let mut cb_args = vec![];
    let rust_invocation = generate_rust_invocation(ctx, &mut cb_args);
    let ident_n_api_fn = ident(&format!("napi_{}", ctx.fn_name()));

    if ctx.is_method() {
        // if function is method, we can't put rust function inside our napi because we need to preserver self
        // in the rust method.
        let napi_fn =
            raw_napi_function_template(ident_n_api_fn, quote! {}, cb_args, rust_invocation);

        quote! {
            #input_fn

            #napi_fn
        }
    } else {
        // otherwise we can put rust function inside to make it tidy
        raw_napi_function_template(
            ident_n_api_fn,
            quote! { #input_fn },
            cb_args,
            rust_invocation,
        )
    }
}

/// generate napi function invocation whether it is method or just free standing function
fn raw_napi_function_template(
    ident_n_api_fn: Ident,
    input_fn: TokenStream,
    rust_args_struct: Vec<TokenStream>,
    rust_invocation: TokenStream,
) -> TokenStream {
    quote! {

        extern "C" fn #ident_n_api_fn(env: ohos_node_bindgen::sys::napi_env,cb_info: ohos_node_bindgen::sys::napi_callback_info) -> ohos_node_bindgen::sys::napi_value
        {
            use ohos_node_bindgen::core::TryIntoJs;
            use ohos_node_bindgen::core::IntoJs;
            use ohos_node_bindgen::core::val::JsCallbackFunction;

            ohos_node_bindgen::core::log::debug!( napi_fn = stringify!(#ident_n_api_fn),"invoking napi function");

            #input_fn

            #(#rust_args_struct)*

            let js_env = ohos_node_bindgen::core::val::JsEnv::new(env);

            #rust_invocation
        }
    }
}
