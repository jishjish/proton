use std::env;
use std::path::PathBuf;
// use polars::prelude::*;


// use proton::transfers::*;
use proton::transfers::ingestion::TransferIngestion; 
use proton::transfers::compression::block_number;

fn main() {

    // get args 
    let args: Vec<String> = env::args().collect();

    // transfer check
    // let schema_check = transfer.check_schema_validity(&PathBuf::from(&args[1])).unwrap();
    // println!("{:?}", schema_check);


    let t = block_number(&PathBuf::from(&args[1])).unwrap();

}
