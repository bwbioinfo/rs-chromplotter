// use crate::plotting::plotly::plotly_chromosome;
use crate::plotting::plotters::plotters_chromosome;
use crate::utils::bedmethyl::read_bedmethyl;
use crate::utils::prepare_data::prepare_data;

pub mod plotting;
pub mod utils;

use clap::Parser;
use std::time::Instant;

/// Simple chromosome scatter plotter
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

    let start = Instant::now();
    let df = read_bedmethyl(&args.bedfile).unwrap();
    let duration = start.elapsed();
    println!("Read bedmethyl in: {:?}", duration);

    let start = Instant::now();
    let filtered = prepare_data(df, &args.chrom).unwrap();
    // If filtered is empty, return a warning and stop
    if filtered.height() == 0 {
        println!("No data found for chromosome {}", &args.chrom);
        return;
    }
    let duration = start.elapsed();
    println!("Prepared data in: {:?}", duration);

    let start = Instant::now();
    plotters_chromosome(filtered, &args.output).unwrap();
    let duration = start.elapsed();
    println!("Plotted in: {:?}", duration);
}

// Add simple tests to the main.rs file

#[cfg(test)]
mod tests {

    use crate::plotting::plotters::plotters_chromosome;
    use crate::utils::bedmethyl::read_bedmethyl;
    use crate::utils::prepare_data::prepare_data;

    #[test]
    fn test_add_fail() {
        let data = read_bedmethyl("test_data/bedmethyl_test.bed.tsv").unwrap();
        prepare_data(data, "chr1").unwrap();
    }

    #[test]
    fn test_plot() {
        let data = read_bedmethyl("test_data/bedmethyl_test.bed.tsv").unwrap();
        let filtered = prepare_data(data, "chr1").unwrap();
        plotters_chromosome(filtered, "test_data/charming_test.png").unwrap();
    }
}

