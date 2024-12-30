use polars::prelude::*;
use std::error::Error;

pub fn prepare_data(df: DataFrame, chromosome: &str) -> Result<DataFrame, Box<dyn Error>> {
    // Filter the DataFrame for a specific chromosome
    let filtered = df.lazy()
        .filter(col("chrom").eq(lit(chromosome)))
        .collect()?;

    // Optionally, aggregate or process data here
    Ok(filtered)
}


#[cfg(test)]
mod tests {
    use crate::read_bedmethyl;

    #[test]
    fn test_add_fail() {
        read_bedmethyl("test_data/aln.bed").unwrap();
    }
}