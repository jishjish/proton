use std::mem;
use polars::prelude::*;
use owo_colors::OwoColorize;


// potential methods
// convert to f64... accurracy loss?
// exponent in tuple
// int then sequential running count --> u16(?)


// pub fn compress_value_string(&mut self, dataset: &DataFrame) -> Result<(Vec<u32>, Vec<u16>), Box<dyn std::error::Error>> {
pub fn compress_value_string(dataset: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {

    let value_strings = dataset.column("value_string").unwrap();
    let value_strings_series = value_strings.as_series().unwrap();

    // println!("{:?}", value_strings_vec);

    let test = value_strings_series.cast(&DataType::UInt32)?;

    // let start_value = test.head(Some(1));
    println!("{}", test);

    // for value in value_strings_series.iter().skip(1) {
    //     println!("value is {}", value);
    // }

    Ok(())
}


