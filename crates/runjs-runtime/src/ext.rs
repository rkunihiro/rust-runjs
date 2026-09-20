use crate::ops::args_ops::op_args;
use crate::ops::fs_ops::op_read_text_file;
use crate::ops::fs_ops::op_write_text_file;

deno_core::extension!(
    runjs_native,
    ops = [op_read_text_file, op_write_text_file, op_args],
    esm_entry_point = "ext:runjs_native/00_bootstrap.js",
    esm = [ dir "src/js", "00_bootstrap.js" ],
);
