use anyhow::Result;
use std::{path::PathBuf, process::Command, str::FromStr};

pub struct BuildOptions<'a> {
    pub layers: Vec<&'a str>,
    pub input: Option<&'a str>,
    pub output: Option<&'a str>,
    pub debug_mode: Option<bool>,
    pub no_build: Option<bool>,
    pub server_language: Option<&'a str>,
}

pub fn build(options: BuildOptions) -> Result<()> {
    let layers = if options.layers.is_empty() {
        vec!["Main"]
    } else {
        options.layers
    };

    let input = PathBuf::from_str(options.input.unwrap_or("."))?;
    let output = PathBuf::from_str(options.output.unwrap_or("."))?;

    let mut cmd = Command::new("idl");

    cmd
        .arg("client")
        .arg(format!("--output={}", output.to_str().unwrap()))
        .arg(format!("--input={}", input.to_str().unwrap()))
        .arg("--language=rust")
        .arg(format!(
            "--layers={}",
            layers.iter().fold("".to_owned(), |acc, &x| {
                format!("{}{}{}", acc, if acc.is_empty() { "" } else { "," }, x)
            })
        ));

    if options.debug_mode.unwrap_or_default() {
        cmd.arg("--debug");
    }

    if options.no_build.unwrap_or_default() {
        cmd.arg("--no-build");
    }

    if let Some(server_language) = options.server_language {
        cmd.arg(format!("--server-language={}", server_language));
    }

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
