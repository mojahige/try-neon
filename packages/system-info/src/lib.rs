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
        Err(_) => String::new(),
    }
}

fn get_os_release() -> String {
    match sys_info::os_release() {
        Ok(release) => release,
        Err(_) => String::new(),
    }
}

fn get_mem_info() -> sys_info::MemInfo {
    match sys_info::mem_info() {
        Ok(info) => info,
        Err(_) => sys_info::MemInfo {
            total: 0,
            free: 0,
            avail: 0,
            buffers: 0,
            cached: 0,
            swap_total: 0,
            swap_free: 0,
        },
    }
}

fn get_host_name() -> String {
    match sys_info::hostname() {
        Ok(name) => name,
        Err(_) => String::new(),
    }
}

fn get_load_average() -> sys_info::LoadAvg {
    match sys_info::loadavg() {
        Ok(info) => info,
        Err(_) => sys_info::LoadAvg {
            one: 0.0,
            five: 0.0,
            fifteen: 0.0,
        },
    }
}

fn get_system_info(mut cx: FunctionContext) -> JsResult<JsObject> {
    let info = cx.empty_object();
    let cpu = cx.empty_object();
    let cpu_quantity = cx.number(get_cpu_num() as f64);
    let cpu_speed = cx.number(get_cpu_speed() as f64);
    let os = cx.empty_object();
    let os_type = cx.string(get_os_type());
    let os_release = cx.string(get_os_release());
    let host = cx.empty_object();
    let host_name = cx.string(get_host_name());
    let memory = cx.empty_object();
    let memory_info = get_mem_info();
    let memory_total = cx.number(memory_info.total as f64);
    let memory_free = cx.number(memory_info.free as f64);
    let load = cx.empty_object();
    let load_average = get_load_average();
    let load_average_one = cx.number(load_average.one);
    let load_average_five = cx.number(load_average.five);
    let load_average_fifteen = cx.number(load_average.fifteen);

    cpu.set(&mut cx, "quantity", cpu_quantity)?;
    cpu.set(&mut cx, "speed", cpu_speed)?;

    os.set(&mut cx, "type", os_type)?;
    os.set(&mut cx, "release", os_release)?;

    host.set(&mut cx, "name", host_name)?;

    memory.set(&mut cx, "total", memory_total)?;
    memory.set(&mut cx, "free", memory_free)?;

    info.set(&mut cx, "cpu", cpu)?;
    info.set(&mut cx, "os", os)?;
    info.set(&mut cx, "host", host)?;
    info.set(&mut cx, "memory", memory)?;
    info.set(&mut cx, "load", load)?;

    Ok(info)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_system_info)?;

    Ok(())
}
