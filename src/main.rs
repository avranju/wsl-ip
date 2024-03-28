use std::{process::Command, str};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    // Run ipconfig
    let output = run_cmd("cmd.exe", &["/c", "ipconfig", "/all"])?;

    let re = Regex::new("vEthernet \\(WSL\\)")?;
    let start = re
        .find(&output)
        .ok_or_else(|| anyhow::anyhow!("No match found"))?
        .start();

    let re = Regex::new("IPv4 Address[. ]*: ([0-9.]+)")?;
    let ip = re
        .captures(&output[start..])
        .ok_or_else(|| anyhow::anyhow!("No match found"))?[1]
        .to_string();

    println!("{ip}");

    Ok(())
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    Ok(
        str::from_utf8(&Command::new(cmd).args(args).output()?.stdout)?
            .trim_end_matches('\n')
            .to_string(),
    )
}
