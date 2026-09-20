use deno_core::op2;
use deno_core::OpState;

/// Passthrough CLI arguments (after `--`), exposed to scripts via `Native.args()`.
pub struct ScriptArgs(pub Vec<String>);

#[op2]
#[serde]
pub fn op_args(state: &mut OpState) -> Vec<String> {
    state.borrow::<ScriptArgs>().0.clone()
}
