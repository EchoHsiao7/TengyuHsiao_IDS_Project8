use polars::prelude::*;
use std::error::Error;
use plotters::prelude::*;
use std::time::Instant;

fn read_data(file: &str) -> Result<DataFrame, Box<dyn Error>> {
    let df = CsvReader::from_path(file)?.infer_schema(Some(100)).finish()?;
    Ok(df)
}

fn generate_summary_statistics(df: &DataFrame, col_name: &str) -> Result<(f64, f64, f64), Box<dyn Error>> {
    let column = df.column(col_name)?;

    let mean_value = column.mean();
    let median_value = column.median();
    let std_value = column.std();

    println!("Summary Statistics for {}:", col_name);
    println!("Mean: {:?}", mean_value);
    println!("Median: {:?}", median_value);
    println!("Standard Deviation: {:?}", std_value);

    Ok((mean_value.unwrap(), median_value.unwrap(), std_value.unwrap()))
}


fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let input_file = "Steam_2024_bestRevenue_1500.csv";
    let col_name = "price"; 

    let df = read_data(input_file)?;
    let column = df.column(col_name)?.f64()?.into_no_null_iter().collect::<Vec<f64>>();
    let (mean, median, std) = generate_summary_statistics(&df, col_name)?;
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    Ok(())
}
