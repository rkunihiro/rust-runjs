use std::path::Path;
use std::rc::Rc;

use anyhow::Context;
use deno_core::resolve_path;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

use crate::ext::runjs_native;
use crate::loader::RunjsModuleLoader;
use crate::ops::args_ops::ScriptArgs;

pub struct RunOptions {
    /// Arguments after `--` on the CLI, exposed to scripts via `Native.args()`.
    pub script_args: Vec<String>,
}

/// Loads and runs `path` as the main ES module. Returns the process exit code:
/// 0 on success, 1 on a module load/transpile error or an uncaught JS exception.
pub async fn run_script(path: &Path, opts: RunOptions) -> anyhow::Result<i32> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(RunjsModuleLoader)),
        extensions: vec![runjs_native::init()],
        ..Default::default()
    });

    runtime
        .op_state()
        .borrow_mut()
        .put(ScriptArgs(opts.script_args));

    let main_module = resolve_path(
        &path.to_string_lossy(),
        &std::env::current_dir().context("failed to get current working directory")?,
    )?;

    let mod_id = match runtime.load_main_es_module(&main_module).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("error: {e}");
            return Ok(1);
        }
    };

    let evaluate = runtime.mod_evaluate(mod_id);

    if let Err(e) = runtime.run_event_loop(Default::default()).await {
        eprintln!("error: {e}");
        return Ok(1);
    }

    if let Err(e) = evaluate.await {
        eprintln!("error: {e}");
        return Ok(1);
    }

    Ok(0)
}
