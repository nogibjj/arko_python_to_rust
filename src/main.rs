use polars::prelude::*;
use std::error::Error;
use std::time::Instant;

fn process_data(file_path: &str) -> Result<f64, Box<dyn Error>> {
    // Read the CSV file into a DataFrame
    let df = CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()?;
    
    // Calculate the average of the 'Close(t)' column
    let avg_close = df
        .column("Close(t)")?
        .f64()?
        .mean()
        .unwrap_or(0.0);

    Ok(avg_close)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Start the timer
    let start = Instant::now();

    // Run the data processing function
    let avg_close = process_data("data/AAPL.csv")?;

    // Measure the elapsed time
    let duration = start.elapsed();

    // Print results
    println!("Average closing price: {}", avg_close);
    println!("Rust Execution time: {:?}", duration);

    Ok(())
}
