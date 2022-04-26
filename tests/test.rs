use std::io::{self, Write};
use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/target.rs"));

#[test]
fn test() {
    let output = Command::new("cargo")
        .args(&[
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
    let expected = format!("target/{}/debug/examples/print\na\nb\nc\n", TARGET);
    #[cfg(windows)]
    let expected = {
        #[rustversion::beta] // 1.61
        const PREFIX: &str = concat!(r"\\?\", env!("CARGO_MANIFEST_DIR"), r"\target");
        #[rustversion::not(beta)]
        const PREFIX: &str = "target";
        format!(
            "{}\\{}\\debug\\examples\\print.exe\na\nb\nc\n",
            PREFIX, TARGET,
        )
    };

    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
}
