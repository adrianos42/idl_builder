use anyhow::Result;
use std::{path::PathBuf, process::Command, str::FromStr};

pub struct BuildOptions<'a> {
    pub layer: Option<&'a str>,
    pub input: Option<&'a str>,
    pub output: Option<&'a str>,
}

pub fn build(options: BuildOptions) -> Result<()> {
    let layer = options.layer.unwrap_or("Main");
    let input = PathBuf::from_str(options.input.unwrap_or("."))?;
    let output = PathBuf::from_str(options.output.unwrap_or("."))?;

    let mut child = Command::new("idl")
        .arg("server")
        .arg(format!("--output={}", output.to_str().unwrap()))
        .arg(format!("--input={}", input.to_str().unwrap()))
        .arg("--language=rust")
        .arg(format!("--layer={}", layer))
        .spawn()?;

    child.wait()?;

    Ok(())
}
