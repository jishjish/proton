use std::mem;
use polars::prelude::*;
use owo_colors::OwoColorize;
use num_bigint::BigUint;
use std::str::FromStr;


// potential methods
// convert to f64... accurracy loss?
// exponent in tuple
// int then sequential running count --> u16(?)


// pub fn compress_value_string(&mut self, dataset: &DataFrame) -> Result<(Vec<u32>, Vec<u16>), Box<dyn std::error::Error>> {
pub fn compress_value_string(dataset: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {

    let value_strings = dataset.column("value_string").unwrap();
    // let value_strings_series = value_strings.as_series().unwrap();
    

    let value_strings_series = value_strings.str().unwrap();

    // let first_str = value_strings.str().unwrap().get(0);

    let mut result: Vec<Option<i128>> = Vec::new();

    for val in value_strings_series.iter() {
        let my_int = val.unwrap().parse::<i128>().unwrap();
        println!("{:?}", my_int);

    }



    // println!("{:?}", my_int);

    Ok(())
}


