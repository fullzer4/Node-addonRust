use neon::prelude::*;

mod functions;
use crate::functions::{hello::hello, sum::sum};

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("sum", sum)?;
    Ok(())
}
