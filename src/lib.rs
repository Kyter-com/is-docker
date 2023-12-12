use neon::prelude::*;
use std::fs;

fn is_docker(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // If /.dockerenv exists, we're in a Docker process
    if fs::metadata("/.dockerenv").is_ok() {
        return Ok(cx.boolean(true));
    }

    // If /proc/self/cgroup contains 'docker', we're in a Docker process
    if let Ok(content) = fs::read_to_string("/proc/self/cgroup") {
        if content.contains("docker") {
            return Ok(cx.boolean(true));
        }
    }

    // If neither condition is met, we're not in a Docker process
    Ok(cx.boolean(false))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("isDocker", is_docker)?;
    Ok(())
}
