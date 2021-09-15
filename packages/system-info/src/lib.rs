use neon::prelude::*;

fn get_cpu_num() -> u32 {
    match sys_info::cpu_num() {
        Ok(num) => num,
        Err(_) => 0 as u32,
    }
}

fn get_cpu_speed() -> u64 {
    match sys_info::cpu_speed() {
        Ok(speed) => speed,
        Err(_) => 0,
    }
}

fn get_os_type() -> String {
    match sys_info::os_type() {
        Ok(os_type) => os_type,
        Err(_) => String::from(""),
    }
}

fn get_system_info(mut cx: FunctionContext) -> JsResult<JsObject> {
    let info = cx.empty_object();
    let cpu = cx.empty_object();
    let cpu_quantity = cx.number(get_cpu_num() as f64);
    let cpu_speed = cx.number(get_cpu_speed() as f64);
    let os = cx.empty_object();
    let os_type = cx.string(get_os_type());

    cpu.set(&mut cx, "quantity", cpu_quantity)?;
    cpu.set(&mut cx, "speed", cpu_speed)?;

    os.set(&mut cx, "type", os_type)?;

    info.set(&mut cx, "cpu", cpu)?;
    info.set(&mut cx, "os", os)?;

    Ok(info)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_system_info)?;

    Ok(())
}
