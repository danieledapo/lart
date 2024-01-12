// TODO: encode numbers as VARINTS

use std::io::{self, Write};

const CMD_START_TAG: u8 = 0xFF;
const CMD_END_TAG: u8 = 0x00;
const DATA_TAG: u8 = 0x0F;

pub fn cmd(cmd: &str, fun: impl FnOnce(&mut Rpc) -> io::Result<()>) -> io::Result<()> {
    if std::env::var("LART").is_err() {
        return Ok(());
    }

    let mut lart = Rpc::stdout();

    lart.data(CMD_START_TAG, cmd.as_bytes())?;
    fun(&mut lart)?;
    lart.data(CMD_END_TAG, &[])?;

    Ok(())
}

pub struct Rpc {
    writer: io::Stdout,
}

impl Rpc {
    pub fn stdout() -> Self {
        Self {
            writer: io::stdout(),
        }
    }

    pub fn kv(&mut self, key: &(impl AsRef<[u8]> + ?Sized), d: &[u8]) -> io::Result<()> {
        self.data(DATA_TAG, key.as_ref())?;
        self.data(DATA_TAG, d)
    }

    fn data(&mut self, tag: u8, d: &[u8]) -> io::Result<()> {
        self.writer.write_all(&[tag])?;
        self.writer
            .write_all(&usize_to_u64(d.len()).to_le_bytes())?;
        self.writer.write_all(d)
    }
}

pub(crate) fn usize_to_u64(n: usize) -> u64 {
    u64::try_from(n).unwrap()
}
