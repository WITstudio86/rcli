use anyhow::Result;
use clap::Parser;
use csv::Reader;
use rcli::{Opts, Player, Subcommand};

fn main() -> Result<()> {
    let opts = Opts::parse();

    match opts.sub {
        Subcommand::CSV(opts) => {
            let mut reader = Reader::from_path(&opts.input)?;
            for read in reader.deserialize() {
                let player: Player = read?;
                println!("{:#?}", player);
            }
        }
    }
    Ok(())
}
