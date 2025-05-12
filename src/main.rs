use std::env;
use std::path::PathBuf;
use polars::prelude::*;


// use proton::transfers::*;
use proton::transfers::ingestion::Transfer; 

fn main() {

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let mut file = std::fs::File::open("data/ethereum__erc20_transfers__10001000_to_10001999.parquet").unwrap();
    // let df = ParquetReader::new(&mut file).finish().unwrap();
    // println!("df: {}", df);


    // transfer check
    let mut transfer = Transfer::new();
    let schema_check = transfer.check_schema_validity(&PathBuf::from(&args[1]));

}
