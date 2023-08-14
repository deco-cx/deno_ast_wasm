// From https://github.com/swc-project/swc/blob/main/crates/binding_core_wasm/src/lib.rs

use deno_ast::parse_module;
use deno_ast::swc::parser::Syntax::Typescript;
use deno_ast::swc::parser::TsConfig;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;
use js_sys::Array;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "parseSync")]
pub fn parse_sync(s: &str) -> Result<JsValue, JsValue> {
    let text_info = SourceTextInfo::new(s.into());
    let parsed_source = parse_module(ParseParams {
        specifier: "file:///my_file.ts".to_string(),
        media_type: MediaType::TypeScript,
        text_info,
        capture_tokens: true,
        maybe_syntax: Some(Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        })),
        scope_analysis: false,
    })
    .expect("should parse");

    let program = JsValue::from_serde(&parsed_source.program()).expect("should be parsed");

    // Create a JavaScript Array to hold the outer array
    let outer_array = Array::new();

    // Iterate through the inner values and create JavaScript arrays for each
    for x in parsed_source.comments().get_vec() {
        let n_object = Object::new();
        js_sys::Reflect::set(
            &n_object,
            &JsValue::from_str("text"),
            &JsValue::from_str(&x.text.to_string()),
        )
        .expect("Failed to set property");

        js_sys::Reflect::set(
            &n_object,
            &JsValue::from_str("span_lo"),
            &JsValue::from_f64(x.span.lo.0.into()),
        )
        .expect("Failed to set property");

        js_sys::Reflect::set(
            &n_object,
            &JsValue::from_str("span_hi"),
            &JsValue::from_f64(x.span.hi.0.into()),
        )
        .expect("Failed to set property");
        // Add the inner array to the outer array
        outer_array.push(&JsValue::from(n_object));
    }

    let comments = JsValue::from(outer_array);
    let js_object = Object::new();

    // Add properties to the JavaScript object
    js_sys::Reflect::set(&js_object, &JsValue::from_str("comments"), &comments)
        .expect("Failed to set property");
    js_sys::Reflect::set(&js_object, &JsValue::from_str("program"), &program)
        .expect("Failed to set property");

    Ok(JsValue::from(js_object))
}
