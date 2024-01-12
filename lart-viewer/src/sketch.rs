use std::{collections::HashMap, process::Command, time::Duration};

use vsvg::Document;

use anyhow::{bail, ensure};

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
        const CMD_START_TAG: u8 = 0xFF;
        const CMD_END_TAG: u8 = 0x00;
        const DATA_TAG: u8 = 0x0F;

        let read_data = |position: &mut usize, tag: u8| -> anyhow::Result<&[u8]> {
            ensure!(*position < output.len(), "truncated data");
            ensure!(output[*position] == tag, "tag mismatch, expected {tag}");
            *position += 1;

            ensure!(*position + 7 < output.len(), "truncated data");
            let sz = u64::from_le_bytes(output[*position..*position + 8].try_into()?);
            let sz = usize::try_from(sz)?;
            *position += 8;

            ensure!(*position + sz < output.len(), "truncated data");
            let d = &output[*position..*position + sz];
            *position += sz;

            Ok(d)
        };

        let mut res = Self::empty();
        let mut position = 0;

        while position < output.len() {
            let cmd = read_data(&mut position, CMD_START_TAG)?;

            let mut kv = HashMap::new();
            loop {
                if output[position] == CMD_END_TAG {
                    position += 9;
                    break;
                }
                let prop = read_data(&mut position, DATA_TAG)?;
                let data = read_data(&mut position, DATA_TAG)?;
                kv.insert(prop, data);
            }

            match cmd {
                b"SVG" => res.parse_svg(&kv)?,
                b"PARM" => res.parse_parm(&kv)?,
                _ => {}
            }
        }

        Ok(res)
    }

    fn parse_svg(&mut self, kv: &HashMap<&[u8], &[u8]>) -> anyhow::Result<()> {
        self.svg_filepath = Some(get_str(kv, b"path")?);

        self.svg = self
            .svg_filepath
            .as_ref()
            .and_then(|p| Document::from_svg(p, true).ok());

        Ok(())
    }

    fn parse_parm(&mut self, kv: &HashMap<&[u8], &[u8]>) -> anyhow::Result<()> {
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

fn get_str(kv: &HashMap<&[u8], &[u8]>, k: &[u8]) -> anyhow::Result<String> {
    let Some(s) = kv.get(k) else {
        bail!("str {} not found", String::from_utf8_lossy(k))
    };

    let s = String::from_utf8(s.to_vec())?;
    Ok(s)
}

fn get_int(kv: &HashMap<&[u8], &[u8]>, k: &[u8]) -> anyhow::Result<i64> {
    let Some(d) = kv.get(k) else {
        bail!("int {} not found", String::from_utf8_lossy(k))
    };

    ensure!(d.len() <= 8, "is not a valid int");

    let n = d
        .iter()
        .rev()
        .fold(0, |acc, byte| (acc << 8_u32) | u64::from(*byte));

    // sign extension
    let shift = (8 - d.len()) * 8;
    let n = (n << shift) as i64 >> shift;

    Ok(n)
}

fn get_uint(kv: &HashMap<&[u8], &[u8]>, k: &[u8]) -> anyhow::Result<u64> {
    let Some(d) = kv.get(k) else {
        bail!("uint {} not found", String::from_utf8_lossy(k))
    };

    ensure!(d.len() <= 8, "is not a valid uint");

    let n = d
        .iter()
        .rev()
        .fold(0, |acc, byte| (acc << 8_u32) | u64::from(*byte));

    Ok(n)
}

fn get_double(kv: &HashMap<&[u8], &[u8]>, k: &[u8]) -> anyhow::Result<f64> {
    let Some(d) = kv.get(k) else {
        bail!("double {} not found", String::from_utf8_lossy(k))
    };

    ensure!(d.len() <= 8, "is not a valid double");

    let d = f64::from_bits(
        d.iter()
            .rev()
            .fold(0, |acc, byte| (acc << 8) | u64::from(*byte)),
    );
    Ok(d)
}
