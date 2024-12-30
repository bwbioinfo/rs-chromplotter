use polars::prelude::*;

// Define the schema for the BEDMethyl file
/// Returns the schema for the Modkit BEDMethyl TSV file
pub fn read_bedmethyl(file_path: &str) -> PolarsResult<DataFrame> {
    let df =  CsvReadOptions::default()
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


#[cfg(test)]
mod tests {
    use crate::read_bedmethyl;

    #[test]
    fn test_add_fail() {
        read_bedmethyl("test_data/aln.bed").unwrap();
    }
}