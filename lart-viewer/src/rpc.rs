use std::collections::HashMap;

use anyhow::{bail, ensure};

const CMD_START_TAG: u8 = 0xFF;
const CMD_END_TAG: u8 = 0x00;
const DATA_TAG: u8 = 0x0F;

pub type KvCmd<'a> = HashMap<&'a [u8], &'a [u8]>;

pub fn parse_cmds(data: &[u8]) -> impl Iterator<Item = anyhow::Result<(&[u8], KvCmd)>> {
    let read_data = |position: &mut usize, tag: u8| -> anyhow::Result<&[u8]> {
        ensure!(*position < data.len(), "truncated data");
        ensure!(data[*position] == tag, "tag mismatch, expected {tag}");
        *position += 1;

        ensure!(*position + 7 < data.len(), "truncated data");
        let sz = u64::from_le_bytes(data[*position..*position + 8].try_into()?);
        let sz = usize::try_from(sz)?;
        *position += 8;

        ensure!(*position + sz < data.len(), "truncated data");
        let d = &data[*position..*position + sz];
        *position += sz;

        Ok(d)
    };

    let read_cmd_kv = move |position: &mut usize| -> anyhow::Result<(&[u8], KvCmd)> {
        let cmd = read_data(position, CMD_START_TAG)?;

        let mut kv = HashMap::new();
        loop {
            if data[*position] == CMD_END_TAG {
                *position += 9;
                break;
            }

            let prop = read_data(position, DATA_TAG)?;
            let data = read_data(position, DATA_TAG)?;
            kv.insert(prop, data);
        }

        Ok((cmd, kv))
    };

    let mut position = 0;

    std::iter::from_fn(move || {
        if position >= data.len() {
            return None;
        }

        match read_cmd_kv(&mut position) {
            Ok(r) => Some(Ok(r)),
            Err(e) => {
                position = data.len();
                Some(Err(e))
            }
        }
    })
}

pub fn get_str(kv: &KvCmd, k: &[u8]) -> anyhow::Result<String> {
    let Some(s) = kv.get(k) else {
        bail!("str {} not found", String::from_utf8_lossy(k))
    };

    let s = String::from_utf8(s.to_vec())?;
    Ok(s)
}

pub fn get_int(kv: &KvCmd, k: &[u8]) -> anyhow::Result<i64> {
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

pub fn get_uint(kv: &KvCmd, k: &[u8]) -> anyhow::Result<u64> {
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

pub fn get_double(kv: &KvCmd, k: &[u8]) -> anyhow::Result<f64> {
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
