use polars::prelude::*;
// use std::path::PathBuf;
// use super::ingestion::TransferIngestion;


// pub fn block_number(filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
pub fn block_number(dataset: DataFrame) -> Result<(), Box<dyn std::error::Error>> {

    let block_numbers = dataset.column("block_number")?
        .u32()?  // Assuming it's u64, adjust type as needed
        .into_iter()
        .filter_map(|v| v) // Handle potential nulls
        .collect::<Vec<u32>>();

    Ok(())
}





pub fn compress(dataset: DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    // block_number(dataset)
    println!("{}", dataset);

    Ok(())
}