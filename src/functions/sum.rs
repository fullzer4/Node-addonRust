use neon::prelude::*;

pub fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let values = cx.argument::<JsArray>(0)?;
    let values: Vec<Handle<JsValue>> = values.to_vec(&mut cx)?;

    let mut sum = 0.0;

    for value in values {
        if let Ok(num) = value.downcast::<JsNumber, _>(&mut cx) {
            sum += num.value(&mut cx);
        }
    }

    Ok(cx.number(sum))
}
