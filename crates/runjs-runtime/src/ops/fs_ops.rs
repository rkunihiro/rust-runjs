use deno_core::op2;
use deno_error::JsErrorBox;

#[op2]
#[string]
pub fn op_read_text_file(#[string] path: String) -> Result<String, JsErrorBox> {
    std::fs::read_to_string(&path)
        .map_err(|e| JsErrorBox::generic(format!("failed to read '{path}': {e}")))
}

// `nofast`: this performs blocking filesystem I/O and allocates, which is not
// appropriate for the V8 fast-call path even though the signature qualifies.
#[op2(nofast)]
pub fn op_write_text_file(
    #[string] path: String,
    #[string] contents: String,
) -> Result<(), JsErrorBox> {
    std::fs::write(&path, contents)
        .map_err(|e| JsErrorBox::generic(format!("failed to write '{path}': {e}")))
}

#[cfg(test)]
mod tests {
    use deno_core::extension;
    use deno_core::JsRuntime;
    use deno_core::RuntimeOptions;

    extension!(test_ext, ops = [super::op_read_text_file, super::op_write_text_file]);

    fn new_runtime() -> JsRuntime {
        JsRuntime::new(RuntimeOptions {
            extensions: vec![test_ext::init()],
            ..Default::default()
        })
    }

    #[test]
    fn read_and_write_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("hello.txt");
        let file_path_str = file_path.to_str().unwrap();

        let mut runtime = new_runtime();
        runtime
            .execute_script(
                "<test>",
                format!(
                    "Deno.core.ops.op_write_text_file({file_path_str:?}, 'hello from js');"
                ),
            )
            .unwrap();

        assert_eq!(
            std::fs::read_to_string(&file_path).unwrap(),
            "hello from js"
        );

        let mut runtime = new_runtime();
        let result = runtime
            .execute_script(
                "<test>",
                format!("Deno.core.ops.op_read_text_file({file_path_str:?})"),
            )
            .unwrap();
        deno_core::scope!(scope, runtime);
        let local = deno_core::v8::Local::new(scope, result);
        let value: String = serde_v8::from_v8(scope, local).unwrap();
        assert_eq!(value, "hello from js");
    }

    #[test]
    fn read_missing_file_errors() {
        let mut runtime = new_runtime();
        let result = runtime
            .execute_script(
                "<test>",
                "try { Deno.core.ops.op_read_text_file('/nonexistent/path/does-not-exist'); 'no-throw' } catch (e) { 'threw' }",
            )
            .unwrap();
        deno_core::scope!(scope, runtime);
        let local = deno_core::v8::Local::new(scope, result);
        let value: String = serde_v8::from_v8(scope, local).unwrap();
        assert_eq!(value, "threw");
    }
}
