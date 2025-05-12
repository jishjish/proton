use polars::prelude::*;
// use std::path::PathBuf;
// use super::ingestion::TransferIngestion;


// pub fn block_number(filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
pub fn block_number(dataset: DataFrame) -> Result<(), Box<dyn std::error::Error>> {

    // let col_len = dataset.select([polars::col("block_number").len()]).unwrap();
    let num_rows = dataset.height();
    println!("col len is: {}", num_rows);

    // let block_numbers = dataset.column("block_number")?
    //     .u32()?  // Assuming it's u64, adjust type as needed
    //     .into_iter()
    //     .filter_map(|v| v) // Handle potential nulls
    //     .collect::<Vec<u32>>();

    let is_unique = dataset.column("block_number").unwrap().unique();

    println!("unique: {:?}", is_unique);


    // println!("{:?}", block_numbers);

    // assert that output is equal in len to input
    // assert_eq!()

    Ok(())
}





pub fn compress(dataset: DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    block_number(dataset)?;
    // println!("{}", dataset);

    Ok(())
}