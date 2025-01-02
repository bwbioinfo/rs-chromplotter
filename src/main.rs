use crate::plotting::plots::plot_chromosome;
use crate::utils::bedmethyl::read_bedmethyl;
use crate::utils::prepare_data::prepare_data;

pub mod plotting;
pub mod utils;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The BedMethyl file to read
    #[arg(short, long)]
    bedfile: String,

    /// The chromosome to plot
    #[arg(short, long)]
    chrom: String,

    // The output file
    #[arg(short, long)]
    output: String,
}


fn main( ) {

    let args = Args::parse();
    println!("{:?}", args);


    let df = read_bedmethyl(&args.bedfile).unwrap();
    let filtered = prepare_data(df, &args.chrom).unwrap();
    // If filtered is empty, return a warning and stop
    if filtered.height() == 0 {
        println!("No data found for chromosome {}", &args.chrom);
        return;
    }
    plot_chromosome(filtered, &args.output).unwrap();
}
