use clap::{Parser, Subcommand};
use std::time::Instant;

use crate::plotting::plotters::plotters_chromosome;
use crate::utils::bedmethyl::read_bedmethyl;
use crate::utils::prepare_data::prepare_data;

pub mod plotting;
pub mod utils;

/// Main CLI parser
#[derive(Parser)]
#[command(name = "myapp", version, about = "A CLI tool for various genomic visusalisations.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Define possible subcommands
#[derive(Subcommand)]
enum Commands {
    /// Generate a chromosome scatter plot
    Chromplot {
        /// The BedMethyl file to read
        #[arg(short, long)]
        bedfile: String,

        /// The chromosome to plot
        #[arg(short, long)]
        chrom: String,

        /// The output file
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chromplot { bedfile, chrom, output } => {
            println!(
                "Running chromplot with file: {}, chromosome: {}, output: {}",
                bedfile, chrom, output
            );

            let start = Instant::now();
            let df = read_bedmethyl(&bedfile).unwrap();
            let duration = start.elapsed();
            println!("Read bedmethyl in: {:?}", duration);

            let start = Instant::now();
            let filtered = prepare_data(df, &chrom).unwrap();
            if filtered.height() == 0 {
                println!("No data found for chromosome {}", chrom);
                return;
            }
            let duration = start.elapsed();
            println!("Prepared data in: {:?}", duration);

            let start = Instant::now();
            plotters_chromosome(filtered, &output).unwrap();
            let duration = start.elapsed();
            println!("Plotted in: {:?}", duration);
        }
    }
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
        plotters_chromosome(filtered, "test_data/test.png").unwrap();
    }
}

