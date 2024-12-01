use std::process::{Command, Stdio};

use crate::template::Day;

pub fn handle(day: Day, year: u16, release: bool, dhat: bool, submit_part: Option<u8>) {
    let bin_name = format!("{}_{:02}", year, day);

    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), bin_name];

    if dhat {
        cmd_args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ]);
    } else if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string());
    }

    println!("Running command: cargo {}", cmd_args.join(" "));

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
