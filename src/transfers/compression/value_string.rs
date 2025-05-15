use std::mem;
use polars::prelude::*;
use owo_colors::OwoColorize;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use std::str::FromStr;

// potential methods
// exponent in tuple
// int then sequential running count --> u16(?)
// normalize

// pub fn compress_value_string(&mut self, dataset: &DataFrame) -> Result<(Vec<u32>, Vec<u16>), Box<dyn std::error::Error>> {
pub fn compress_value_string(dataset: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {

    // Distill value_string column from dataset and unwrap the str's
    let value_strings = dataset.column("value_string").unwrap();
    let value_strings_series = value_strings.str().unwrap();

    // Parse to BigDecimal
    // Set scaled value for easier reference, push to vec
    let mut result: Vec<BigDecimal> = Vec::new();
    for val in value_strings_series.iter() {
        let decimal = BigDecimal::from_str(val.unwrap()).unwrap();
        let divisor = BigDecimal::from(10);
        let scaled_value = decimal / divisor;
        // println!("{}", scaled_value.to_string());
        result.push(scaled_value);
    }

    // Convert result to f64 and normalize dataset
    let result_f64: Vec<f64> = result.iter().map(|x| x.to_f64().unwrap_or(0.0)).collect();
    let squared_sum: f64 = result_f64.iter().map(|x| x * x).sum();
    let sqrt_of_sum = squared_sum.sqrt();
    let normalized_vec: Vec<f64> = result_f64.iter().map(|x| x / sqrt_of_sum).collect();
    let sum_of_squares: f64 = normalized_vec.iter().map(|x| x * x).sum();
    println!("Sum of squares: {}", sum_of_squares); // Should be ≈ 1.0

    // Calculate size of original string 
    let original_str_len = value_strings_series.iter()
        .map(|s| s.unwrap().len())
        .sum::<usize>();
    // Calculate size of compresed vec
    let compressed_size = normalized_vec.capacity() * mem::size_of::<u16>();

    // Print comparisons to terminal
    println!("Original block index: {} bytes", original_str_len.red());
    println!("Compressed block index: {} bytes", compressed_size.green());
    println!("Compression Ratio: {:.2}", original_str_len as f64 / compressed_size as f64);

    Ok(())
}


