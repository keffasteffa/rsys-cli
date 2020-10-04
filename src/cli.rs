use super::{
    cmd::{RsysCmd, RsysOpt},
    util::PrintFormat,
};
use rsys::{Result, Rsys};
use structopt::StructOpt;

pub struct RsysCli {
    pub opts: RsysOpt,
    pub system: Rsys,
}
impl RsysCli {
    pub fn new() -> RsysCli {
        RsysCli {
            opts: RsysOpt::from_args(),
            system: Rsys::new(),
        }
    }

    pub fn main(&self) -> Result<()> {
        if let Some(cmd) = &self.opts.cmd {
            match cmd {
                RsysCmd::Get {
                    property,
                    json,
                    yaml,
                    pretty,
                } => {
                    let format = if *json {
                        PrintFormat::Json
                    } else if *yaml {
                        PrintFormat::Yaml
                    } else {
                        PrintFormat::Normal
                    };
                    self.get(property, format, *pretty)?
                }
                RsysCmd::Dump {
                    json,
                    yaml,
                    pretty,
                    cpu,
                    memory,
                    network,
                    storage,
                    mounts,
                    all,
                    stats,
                    processes,
                } => {
                    let format = if *json {
                        PrintFormat::Json
                    } else if *yaml {
                        PrintFormat::Yaml
                    } else {
                        PrintFormat::Normal
                    };
                    self.dump(
                        format, *pretty, *cpu, *memory, *network, *storage, *mounts, *all, *stats, *processes,
                    )?
                }
                RsysCmd::Watch {
                    pretty,
                    cpu,
                    memory,
                    network,
                    storage,
                    all,
                    stats,
                    duration,
                    interval,
                } => self.watch(
                    *pretty, *cpu, *memory, *network, *storage, *all, *stats, *duration, *interval,
                )?,
                RsysCmd::Graph { graph: cmd } => self.graph(cmd.clone()),
            }
        }

        Ok(())
    }
}
