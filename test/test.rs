use polars::prelude::*;
use std::error::Error;
use assert_approx_eq::assert_approx_eq;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn create_sample_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    writeln!(file, "Date,Close(t)")?;
    writeln!(file, "2024-01-01,150.0")?;
    writeln!(file, "2024-01-02,152.0")?;
    writeln!(file, "2024-01-03,148.0")?;
    writeln!(file, "2024-01-04,151.0")?;
    writeln!(file, "2024-01-05,149.0")?;
    Ok(())
}

fn process_data(file_path: &str) -> Result<f64, Box<dyn Error>> {
    let df = CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()?;
    
    let avg_close = df
        .column("Close(t)")?
        .f64()?
        .mean()
        .unwrap_or(0.0);

    Ok(avg_close)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() -> Result<(), Box<dyn Error>> {
        let test_file_path = "test_data.csv";
        
        // Create a sample CSV file for testing
        create_sample_csv(test_file_path)?;

        // Process data and get the average closing price
        let avg_close = process_data(test_file_path)?;

        // Check if the average is as expected
        assert_approx_eq!(avg_close, 150.0, 1e-2);

        // Clean up
        std::fs::remove_file(test_file_path)?;

        Ok(())
    }
}
