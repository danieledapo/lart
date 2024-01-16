use std::{collections::HashMap, process::Command, time::Duration};

use vsvg::Document;

use anyhow::bail;

use crate::schema::{Manifest, Parm};

#[derive(Clone, Debug)]
pub struct SketchInput {
    pub timeout: Duration,
    pub cmd: Vec<String>,
    pub parameters: Manifest,
}

#[derive(Clone, Debug)]
pub struct SketchOutput {
    pub manifest: Manifest,
    pub svg_filepath: Option<String>,
    pub svg: Option<Document>,
}

pub fn sketch_run(sketch_input: &SketchInput) -> anyhow::Result<SketchOutput> {
    let output = Command::new(&sketch_input.cmd[0])
        .args(&sketch_input.cmd[1..])
        .env("LART", "1")
        .args(sketch_input.parameters.iter().flat_map(|(p, schema)| {
            [
                format!("--{p}"),
                match schema {
                    Parm::String { value } => value.clone(),
                    Parm::Bool { value } => value.to_string(),
                    Parm::Int { value, .. } => value.to_string(),
                    Parm::UInt { value, .. } => value.to_string(),
                    Parm::Double { value, .. } => value.to_string(),
                    Parm::Choice { value, .. } => value.clone(),
                },
            ]
        }))
        .output()?;

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        bail!("<generation failed>");
    }

    SketchOutput::from_output(&output.stdout)
}

impl SketchOutput {
    pub fn empty() -> Self {
        Self {
            manifest: Manifest::new(),
            svg_filepath: None,
            svg: None,
        }
    }

    fn from_output(output: &[u8]) -> anyhow::Result<Self> {
        let mut res = Self::empty();

        for cmd in crate::rpc::parse_cmds(output) {
            let (cmd, kv) = cmd?;
            match cmd {
                b"SVG" => res.parse_svg(&kv)?,
                b"PARM" => res.parse_parm(&kv)?,
                _ => {}
            }
        }

        Ok(res)
    }

    fn parse_svg(&mut self, kv: &HashMap<&[u8], &[u8]>) -> anyhow::Result<()> {
        self.svg_filepath = Some(crate::rpc::get_str(kv, b"path")?);

        self.svg = self
            .svg_filepath
            .as_ref()
            .and_then(|p| Document::from_svg(p, true).ok());

        Ok(())
    }

    fn parse_parm(&mut self, kv: &HashMap<&[u8], &[u8]>) -> anyhow::Result<()> {
        use crate::rpc::{get_double, get_int, get_str, get_uint};

        let parm = get_str(kv, b"name")?;
        let ty = get_str(kv, b"type")?;

        let p = if ty == "bool" {
            Parm::Bool {
                value: kv[&b"default"[..]][0] != 0,
            }
        } else if ty == "int" {
            Parm::Int {
                value: get_int(kv, b"default")?,
                min: get_int(kv, b"min")?,
                max: get_int(kv, b"max")?,
            }
        } else if ty == "uint" {
            Parm::UInt {
                value: get_uint(kv, b"default")?,
                min: get_uint(kv, b"min")?,
                max: get_uint(kv, b"max")?,
            }
        } else if ty == "double" {
            Parm::Double {
                value: get_double(kv, b"default")?,
                min: get_double(kv, b"min")?,
                max: get_double(kv, b"max")?,
            }
        } else if ty == "string" {
            Parm::String {
                value: get_str(kv, b"default")?,
            }
        } else if ty == "choice" {
            Parm::Choice {
                value: get_str(kv, b"default")?,
                choices: anyhow::Result::from_iter(
                    (0..get_uint(kv, b"len")?).map(|i| get_str(kv, &i.to_le_bytes())),
                )?,
            }
        } else {
            return Ok(());
        };

        self.manifest.push((parm, p));

        Ok(())
    }
}
