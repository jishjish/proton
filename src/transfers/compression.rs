use polars::prelude::*;
use std::path::PathBuf;
use super::ingestion::TransferIngestion;


// pub fn block_number(data: DataFrame) -> Result<(), Box<dyn std::error::Error>> {
pub fn block_number(filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut transfer = TransferIngestion::new();
    let schema_check = transfer.check_schema_validity(filepath).unwrap();

    println!("data is {}", schema_check);
    Ok(())
}