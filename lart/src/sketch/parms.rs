use std::{env::Args, io, iter::Skip, mem, ops::RangeInclusive, str::FromStr};

use crate::{
    rpc::{usize_to_u64, Rpc},
    Page,
};

pub trait Parm {
    // An error is a panic when parsing
    fn parse_args(&mut self, args: &mut CliTokens);
    fn send_schema(&self, rpc: &mut Rpc) -> io::Result<()>;
}

pub enum Schema {
    String(String),
    Int(i128, RangeInclusive<i128>),
    Double(f64, RangeInclusive<f64>),
    Bool(bool),
    Choice(Choice),
}

#[derive(Clone, Debug)]
pub struct Choice {
    val: String,
    choices: &'static [&'static str],
}

pub struct CliTokens {
    value: String,
    args: Skip<Args>,
}

#[macro_export]
macro_rules! sketch_parms {
    ($($name: ident: $t: ty = $def: expr),* $(,)?) => {
        use std::io::{self, Write};

        #[derive(Debug)]
        struct Parms {
            $(pub $name: $t),*
        }

        impl Default for Parms {
            fn default() -> Self {
                Parms {
                    $(
                        $name: $def,
                    )*
                }
            }
        }

        impl Parms {
            pub fn from_cli() -> Self {
                let mut args = CliTokens::from_env();

                let mut parms = Parms::default();

                $(
                    $crate::rpc::cmd("PARM", |rpc| {
                        rpc.kv("name", stringify!($name).as_bytes())?;
                        parms.$name.send_schema(rpc)
                    }).unwrap();
                )*

                while let Some(option) = args.next() {
                    let Some(option_name) = option.strip_prefix("--") else {
                        // given that only long options are supported it's
                        // either that or an unparsed value
                        continue;
                    };

                    $(
                        if option_name == stringify!($name) {
                            parms.$name.parse_args(&mut args);
                        }
                    )*
                }

                parms
            }
        }
    };
}

impl CliTokens {
    pub fn from_env() -> Self {
        Self {
            value: String::new(),
            args: std::env::args().skip(1),
        }
    }

    fn parsed_next<T: FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.next().unwrap().parse().unwrap()
    }
}

impl Iterator for CliTokens {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.value.is_empty() {
            return Some(mem::take(&mut self.value));
        }

        let mut option = self.args.next()?;

        if let Some(i) = option.find('=') {
            self.value = option.split_off(i);
            self.value.remove(0);
        } else {
            self.value.clear();
        }

        Some(option)
    }
}

impl Parm for bool {
    fn parse_args(&mut self, args: &mut CliTokens) {
        let n = args.next().unwrap().to_lowercase();
        if n == "true" || n == "1" {
            *self = true;
        } else if n == "false" || n == "0" {
            *self = false;
        } else {
            panic!("{n} is a valid bool, note that even boolean flags require an explicit value");
        }
    }

    fn send_schema(&self, rpc: &mut Rpc) -> io::Result<()> {
        rpc.kv("type", "bool".as_bytes())?;
        rpc.kv("default", &[*self as u8])
    }
}

impl Parm for String {
    fn parse_args(&mut self, args: &mut CliTokens) {
        let p = args.parsed_next::<String>();

        let p = match p.strip_prefix('"') {
            Some(p) => p,
            _ => &p,
        };

        let p = match p.strip_suffix('"') {
            Some(p) => p,
            _ => p,
        };

        *self = p.to_string();
    }

    fn send_schema(&self, rpc: &mut Rpc) -> io::Result<()> {
        rpc.kv("type", "string".as_bytes())?;
        rpc.kv("default", self.as_bytes())
    }
}

impl Choice {
    pub fn new(s: &'static str, choices: &'static [&'static str]) -> Self {
        Self {
            val: s.to_string(),
            choices,
        }
    }

    pub fn value(&self) -> &str {
        &self.val
    }
}

impl Parm for Choice {
    fn parse_args(&mut self, args: &mut CliTokens) {
        self.val.parse_args(args);
        if !self.choices.contains(&self.val.as_str()) {
            panic!("{} is not a valid choice", self.val);
        }
    }

    fn send_schema(&self, rpc: &mut Rpc) -> io::Result<()> {
        rpc.kv("type", "choice".as_bytes())?;
        rpc.kv("default", self.val.as_bytes())?;
        rpc.kv("len", &usize_to_u64(self.choices.len()).to_le_bytes())?;
        for (choice, i) in self.choices.iter().zip(0_u64..) {
            rpc.kv(&i.to_le_bytes(), choice.as_bytes())?;
        }
        Ok(())
    }
}

impl Parm for Page {
    fn parse_args(&mut self, args: &mut CliTokens) {
        let mut p = String::new();
        p.parse_args(args);

        *self = Self::STD_SIZES
            .iter()
            .find(|(s, _)| s == &p)
            .expect("invalid page")
            .1
            .clone();
    }

    fn send_schema(&self, rpc: &mut Rpc) -> io::Result<()> {
        Choice::new(
            Self::STD_SIZES
                .iter()
                .find(|(_, d)| d == self)
                .expect("invalid page")
                .0,
            &["A6", "A5", "A4", "A3", "A2", "A1", "A0"],
        )
        .send_schema(rpc)
    }
}

macro_rules! impl_num_arg {
    ($t: ty, $lart_t: ident) => {
        impl Parm for $t {
            fn parse_args(&mut self, args: &mut CliTokens) {
                *self = args.parsed_next();
            }

            fn send_schema(&self, rpc: &mut Rpc) -> io::Result<()> {
                rpc.kv("type", stringify!($lart_t).as_bytes())?;
                rpc.kv("min", &<$t>::MIN.to_le_bytes())?;
                rpc.kv("max", &<$t>::MAX.to_le_bytes())?;
                rpc.kv("default", &self.to_le_bytes())
            }
        }
    };
}

impl_num_arg!(u8, uint);
impl_num_arg!(u16, uint);
impl_num_arg!(u32, uint);
impl_num_arg!(u64, uint);
impl_num_arg!(u128, uint);
impl_num_arg!(usize, uint);

impl_num_arg!(i8, int);
impl_num_arg!(i16, int);
impl_num_arg!(i32, int);
impl_num_arg!(i64, int);
impl_num_arg!(i128, int);
impl_num_arg!(isize, int);

impl_num_arg!(f32, double);
impl_num_arg!(f64, double);
