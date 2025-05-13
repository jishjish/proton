use std::fs::File;
use std::path::PathBuf;
use polars::prelude::*;

// internal code
use super::ingestion::TransferIngestion;
use crate::transfers::compression::{RLECompressedBlockNumberSeries, RLECompressedTransactionIndexSeries};


pub struct Transfer {
    // pub og_df: DataFrame,            // incoming dataset (from filepath)
    // pub compressed_df: DataFrame,    // dataset after compression
    pub output_filepath: PathBuf,       // filepath for wrting compressed file
}


impl Transfer {

    pub fn new() -> Self {
        Self {
            // og_df: DataFrame::default(),          // dataframe after schema check
            // compressed_df: DataFrame::default(),  // compresed dataframe; pre file write
            output_filepath: PathBuf::new(),      // output filepath; 
        }
    }

    /// Set new filepath based on incoming path. Will be same location, just with prefix "PROTON_"
    pub fn _update_path(&mut self, filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // set output file path; add 'PROTON_' designation
        let mut path = PathBuf::from(filepath);
        let filename = path.file_name().unwrap().to_string_lossy();
        // add designation to string lossy filename
        let amended_filename = format!("PROTON_{}", filename);
        // set file name, push to mut path
        path.set_file_name(amended_filename);
        self.output_filepath = path;
        Ok(())
    }

    pub fn orchestrate(&mut self, filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        
        /*********** OVERALL PROCESS ***********/
        // 1) Cross reference incoming dataset against valid schema
        // 2) Pass columns through compression algorithms
        // 3) Generate dataframes, stack dataframes
        // 4) Write output via `_update_path` 

        // Instantiate TransferIngestion; validate schema against transfer dataset
        let mut transfer = TransferIngestion::new();
        let schema_check = transfer.check_schema_validity(filepath).unwrap();
        // println!("schema check is {}", schema_check);

        // NOTE: Ensure congruency in df creation for various column compressions
        /* Given there are multiple compression algorithms being used it is 
           imperative to use the proper creator function via transfers/utils.rs.
           For example, RLE uses a value/count approach and will likely need 
           different approaches to construction. */
        
        // 1) block_number: rle compression
        let mut block_compression = RLECompressedBlockNumberSeries::new();
        let compressed_blocks = block_compression.compress_block_number(&schema_check);
        println!("Compressed blocks: {:?}", compressed_blocks);

        // 2) transaction_index: rle compression
        let mut transaction_compression = RLECompressedTransactionIndexSeries::new();
        let compressed_trans_index = transaction_compression.compress_transaction_index(&schema_check);
        println!("Compressed transaction index: {:?}", compressed_trans_index);



        // example code for writing stacked parquet dfs
        // TODO: solve for index values
        // let mut df1 = df!(
        //     "id" => &[1, 2, 3],
        //     "name" => &["a", "b", "c"]
        // )?;
        
        // // Second dataframe
        // let df2 = df!(
        //     "id" => &[4, 5, 6, 7],
        //     "name" => &["d", "e", "f", "g"]
        // )?;
        
        // // Concatenate
        // let mut combined = df1.vstack(&df2)?;
        
        // // Write to Parquet
        // let mut file = File::create("data/combined.parquet")?;
        // ParquetWriter::new(&mut file).finish(&mut combined)?;

        Ok(())
    }
}
