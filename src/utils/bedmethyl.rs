pub mod bedmethyl {
    use polars::prelude::*;

    // Define the schema for the BEDMethyl file
    /// Returns the schema for the Modkit BEDMethyl TSV file
    pub fn read_bedmethyl(file_path: &str) -> PolarsResult<DataFrame> {
        let df = CsvReadOptions::default()
            .with_has_header(true)
            .map_parse_options(|parse_options| parse_options.with_separator(b'\t'))
            .try_into_reader_with_file_path(Some(file_path.into()))
            .unwrap()
            .finish()
            .unwrap();
        println!("{:?}", "All data loaded");
        println!("{:?}", df);
        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    // use super::super::preprocess::preprocess::prepare_data;
    use super::bedmethyl::read_bedmethyl;

    #[test]
    fn test_read_bedmethyl_valid_file() {
        let file_path = "test_data/bedmethyl_test.bed.tsv";
        let df_result = read_bedmethyl(file_path);

        assert!(df_result.is_ok(), "Expected Ok(DataFrame), got Err");
        let df = df_result.unwrap();

        // Optionally check for column names or row count
        assert!(df.height() > 0, "Expected non-empty DataFrame");
        assert!(
            df.get_column_names()
                .iter()
                .map(|s| s.as_str())
                .any(|name| name == "#chrom"),
            "Missing 'chrom' column"
        );
    }
}
