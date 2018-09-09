#[macro_use]
extern crate neon;
extern crate num_cpus;


use neon::js::JsNumber;
use neon::prelude::*;

// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     Ok(cx.string("hello node"))
// }

fn threading_hint(call: Call) -> &'static JsResult<JsNumber> {
    return Ok(JsNumber::new(call.scope, num_cpus::get() as f64))
}

register_module!(mut cx, {
    // cx.export_function("hello", hello);
    cx.export("threading_hint", threading_hint)

});
