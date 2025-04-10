pub mod preprocess {
    use polars::prelude::*;
    use std::error::Error;

    pub fn prepare_data(df: DataFrame, chromosome: &str) -> Result<DataFrame, Box<dyn Error>> {
        // Filter the DataFrame for a specific chromosome
        let filtered = df
            .lazy()
            .filter(col("#chrom").eq(lit(chromosome)))
            .collect()?;
        // Optionally, aggregate or process data here
        println!("Data for {} loaded", chromosome);
        println!("{:?}", filtered);
        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::super::bedmethyl::bedmethyl::read_bedmethyl;
    use super::preprocess::prepare_data;

    #[test]
    fn test_add_fail() {
        let data = read_bedmethyl("test_data/bedmethyl_test.bed.tsv").unwrap();
        prepare_data(data, "chr1").unwrap();
    }
}
