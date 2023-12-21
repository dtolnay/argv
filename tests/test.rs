use std::io::{self, Write};
use std::process::Command;
use target_triple::TARGET;

#[test]
fn test() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--example",
            "print",
            "--target",
            TARGET,
            "--",
            "a",
            "b",
            "c",
        ])
        .output()
        .expect("failed to execute process");
    io::stderr().lock().write_all(&output.stderr).unwrap();
    assert!(output.status.success());

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let expected = format!("target/{TARGET}/debug/examples/print\na\nb\nc\n");
    #[cfg(windows)]
    let expected = format!("target\\{TARGET}\\debug\\examples\\print.exe\na\nb\nc\n");

    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
}
