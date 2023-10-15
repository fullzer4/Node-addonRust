use neon::prelude::*;

pub fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello from rust"))
}
