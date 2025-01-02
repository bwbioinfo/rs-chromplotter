use charming::{
    component::{Title, Axis},
    series::Scatter,
    Chart,
    ImageRenderer,
    dz,
    theme::Theme
};

use polars::prelude::*;
use std::{error::Error, vec};

pub fn plot_chromosome(data: DataFrame, output_path: &str) -> Result<(), Box<dyn Error>> {

    // Extract the columns we need
    let start_position: Vec<Option<i64>> =  data.column("chromStart").unwrap().i64().unwrap().into_iter().collect();
    let percent_modified = data.column("percent_modified").unwrap().f64().unwrap().to_vec();
    // Transpose the data into a format that can be used by the chart
    let datapoints = dz!(start_position, percent_modified);

    // get the chromosome name as a single string
    let chromosome_name = 
        data
        .column("chrom")
        .unwrap()
        .str()
        .unwrap()
        .get(0)
        .unwrap();

    // Create the chart
    let chart = Chart::new()
    // set the chromosome name as the title
    .title(Title::new().text(format!("Chromosome {} Modification Frequency", chromosome_name)))
    .x_axis(Axis::new())
    .y_axis(Axis::new())
    .series(
        Scatter::new()
        .symbol_size(10)
        .data(datapoints)
    );

    // Render the chart
    let mut renderer = 
        ImageRenderer::new(1500, 200)
        .theme(Theme::Dark);
    
    _ = renderer.save(&chart, output_path);

    println!("Saved to: {}", output_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::read_bedmethyl;
    use crate::prepare_data;
    use crate::plot_chromosome;

    #[test]
    fn test_add_fail() {
        let data = read_bedmethyl("test_data/bedmethyl_test.bed.tsv").unwrap();
        let filtered = prepare_data(data, "chr1").unwrap();
        plot_chromosome(filtered, "test_data/chr1.svg").unwrap();
    }
}