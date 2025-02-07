use plotters::prelude::*;
use polars::prelude::*;
use std::error::Error;

pub fn plotters_chromosome(data: DataFrame, output_path: &str) -> Result<(), Box<dyn Error>> {

    let chromosome_name = data
    .column("chrom")
    .unwrap()
    .str()
    .unwrap()
    .get(0)
    .unwrap();

    // Extract the columns we need
    let start_position: Vec<i64> = data
        .column("chromStart")
        .unwrap()
        .i64()
        .unwrap()
        .into_no_null_iter()
        .collect();
    //get max x value 
    let x_max_value = start_position.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().abs();

    let percent_modified: Vec<f64> = data
        .column("percent_modified")
        .unwrap()
        .f64()
        .unwrap()
        .into_no_null_iter()
        .collect();
    // get max y value and round up as integer
    let y_max_value = percent_modified.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().ceil();

    // Create a vector of tuples
    let chrom_points: Vec<(i64, f64)> = start_position
        .iter()
        .zip(percent_modified.iter())
        .map(|(x, y)| {
            (
                *x,
                *y,
            )
        })
        .collect();

    let root = BitMapBackend::new(output_path, (3000, 500))
        .into_drawing_area();

    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(10)
        .y_label_area_size(10)   
        .set_label_area_size(LabelAreaPosition::Left, 75)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .margin(50)
        // Set plot name based on input chromosome
        .caption( 
            format!("Chromosome {} Modification Frequency", chromosome_name),
            ("sans-serif", 40)
        )
        .build_cartesian_2d( 0i64..x_max_value, 0f64..y_max_value )?;

    scatter_ctx
        .configure_mesh()
        .x_desc("Chromosome Position")
        .y_desc("Percent Modified")
        .axis_desc_style(("sans-serif", 25))
        // .disable_x_mesh()
        // .disable_y_mesh()
        .draw()?;

    scatter_ctx.draw_series(
        chrom_points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLUE.filled())),
    )?;
    // Message to user
    println!("Plot saved to {}", output_path);

    Ok(())
}

// test the plotting function
#[cfg(test)]
mod tests {
    use polars::prelude::*;
    use super::*;

    #[test]
    fn test_plotters() {

        let chrom = Series::new("chrom".into() , &["chr1"; 10]);
        let start = Series::new("chromStart".into(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).cast(&DataType::Int64).unwrap();
        let modified = Series::new("percent_modified".into(), &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]);

        //construct a polars DataFrame
        let data = DataFrame::new(vec![
            chrom.into(),
            start.into(),
            modified.into(),
        ])
        .unwrap();
        // print the first 10 elements
        println!("{:?}", &data.head(Some(10)));
        plotters_chromosome(data, "test_data/plotters_test.png").unwrap();
    }
}
//


