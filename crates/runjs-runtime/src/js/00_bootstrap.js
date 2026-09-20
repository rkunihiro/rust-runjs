// Wraps the raw `Deno.core.ops.op_*` calls in a friendlier `Native.*` namespace
// for scripts to call. This is the only surface user scripts are expected to use
// for native functionality - add new wrappers here as new ops are registered.
const core = globalThis.Deno.core;

globalThis.Native = Object.freeze({
  readTextFile(path) {
    return core.ops.op_read_text_file(path);
  },
  writeTextFile(path, contents) {
    return core.ops.op_write_text_file(path, contents);
  },
  args() {
    return core.ops.op_args();
  },
});
