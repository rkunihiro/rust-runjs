use assert_cmd::Command;
use predicates::prelude::*;

fn fixture(name: &str) -> String {
    format!(
        "{}/../../tests/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

#[test]
fn runs_plain_js_with_builtins() {
    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("hello.js"))
        .assert()
        .success()
        .stdout(predicate::str::contains("hello from js"))
        .stdout(predicate::str::contains("3"))
        .stdout(predicate::str::contains("{\"a\":1}"));
}

#[test]
fn transpiles_and_runs_typescript() {
    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("hello.ts"))
        .assert()
        .success()
        .stdout(predicate::str::contains("hello, typescript"));
}

#[test]
fn uncaught_exception_exits_nonzero() {
    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("throws.js"))
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("boom"));
}

#[test]
fn supports_local_es_module_imports() {
    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("import_module.ts"))
        .assert()
        .success()
        .stdout(predicate::str::contains("36"));
}

#[test]
fn native_fs_read_write_and_args_passthrough() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("out.txt");

    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("native_fs.js"))
        .arg("--")
        .arg(target.to_str().unwrap())
        .arg("hello native fs")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello native fs"));

    assert_eq!(
        std::fs::read_to_string(&target).unwrap(),
        "hello native fs"
    );
}

#[test]
fn check_flag_is_explicitly_unimplemented() {
    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("hello.ts"))
        .arg("--check")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("not implemented"));
}

#[test]
fn missing_script_is_a_usage_error() {
    Command::cargo_bin("runjs")
        .unwrap()
        .arg(fixture("does-not-exist.js"))
        .assert()
        .failure()
        .code(2);
}
