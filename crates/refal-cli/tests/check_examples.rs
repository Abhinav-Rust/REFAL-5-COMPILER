use std::process::Command;

fn refal_bin() -> &'static str {
    env!("CARGO_BIN_EXE_refal")
}

fn workspace_path(path: &str) -> String {
    format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), path)
}

#[test]
fn accepts_positive_examples() {
    for path in [
        "examples/identity.ref",
        "examples/hello.ref",
        "examples/condition.ref",
        "examples/extern.ref",
        "examples/classic-syntax.ref",
    ] {
        let output = Command::new(refal_bin())
            .args(["check", &workspace_path(path)])
            .output()
            .expect("run refal binary");

        assert!(
            output.status.success(),
            "{path} should pass\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn rejects_negative_examples() {
    for path in [
        "examples/bad-unresolved-call.ref",
        "examples/bad-unbound-variable.ref",
        "examples/bad-lowercase-identifier.ref",
        "examples/bad-malformed-real.ref",
        "examples/bad-call-in-pattern.ref",
    ] {
        let output = Command::new(refal_bin())
            .args(["check", &workspace_path(path)])
            .output()
            .expect("run refal binary");

        assert!(
            !output.status.success(),
            "{path} should fail\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn reports_line_and_column_for_lex_error() {
    let output = Command::new(refal_bin())
        .args([
            "check",
            &workspace_path("examples/bad-lowercase-identifier.ref"),
        ])
        .output()
        .expect("run refal binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "lex error at 1:1: Classic Refal-5 identifiers must start with an uppercase letter"
        ),
        "unexpected stderr:\n{stderr}"
    );
}

#[test]
fn reports_line_and_column_for_malformed_real_number() {
    let output = Command::new(refal_bin())
        .args(["check", &workspace_path("examples/bad-malformed-real.ref")])
        .output()
        .expect("run refal binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("lex error at 2:5: real number requires digits after decimal point"),
        "unexpected stderr:\n{stderr}"
    );
}

#[test]
fn reports_line_and_column_for_pattern_call_error() {
    let output = Command::new(refal_bin())
        .args(["check", &workspace_path("examples/bad-call-in-pattern.ref")])
        .output()
        .expect("run refal binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("semantic error at 2:3: function calls are not allowed in patterns"),
        "unexpected stderr:\n{stderr}"
    );
}

#[test]
fn reports_line_and_column_for_semantic_error() {
    let output = Command::new(refal_bin())
        .args(["check", &workspace_path("examples/bad-unresolved-call.ref")])
        .output()
        .expect("run refal binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("semantic error at 2:5: unresolved function call `Missing`"),
        "unexpected stderr:\n{stderr}"
    );
}

#[test]
fn runs_program_and_prints_prout_output() {
    let output = Command::new(refal_bin())
        .args(["run", &workspace_path("examples/hello.ref")])
        .output()
        .expect("run refal binary");

    assert!(
        output.status.success(),
        "run should pass\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, Refal\n");
}
