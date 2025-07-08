use clap::Parser;
use core::fmt;
use std::path::PathBuf;
use tabled::Tabled;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum Modified {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Ord, Clone, Copy)]
pub enum Size {
    Byte(u64),
    KiloByte(u64),
    MegaByte(u64),
    GigaByte(u64),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Size::Byte(b) => {
                write!(f, "{}b", b)
            }
            Size::KiloByte(b) => {
                write!(f, "{}Kb", b)
            }
            Size::MegaByte(b) => {
                write!(f, "{}Mb", b)
            }
            Size::GigaByte(b) => {
                write!(f, "{}Gb", b)
            }
        }
    }
}

impl fmt::Display for Modified {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modified::Seconds(t) => {
                write!(f, "{}sec", t)
            }
            Modified::Minutes(t) => write!(f, "{}min", t),
            Modified::Hours(t) => write!(f, "{}hrs", t),
            Modified::Days(t) => write!(f, "{}days", t),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub dir: PathBuf,

    #[arg(long)]
    pub by_size: bool,

    #[arg(long)]
    pub by_modified: bool,
}

#[derive(Tabled)]
pub struct FileMeta {
    pub name: String,
    pub size: Size,
    pub modified: Modified,
}
