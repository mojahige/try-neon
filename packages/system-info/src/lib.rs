use neon::prelude::*;

fn get_cpu_num(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let result: f64 = match sys_info::cpu_num() {
        Ok(num) => num as f64,
        Err(_) => 0 as f64,
    };

    Ok(cx.number(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_cpu_num)?;
    Ok(())
}
