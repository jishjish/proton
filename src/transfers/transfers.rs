use std::path::PathBuf;
use polars::prelude::*;

// internal code
use super::ingestion::TransferIngestion;
use crate::transfers::compression::{RLECompressedBlockNumberSeries};


pub struct Transfer {
    pub og_df: DataFrame,            // incoming dataset (from filepath)
    pub compressed_df: DataFrame,    // dataset after compression
    pub output_filepath: PathBuf,    // filepath for wrting compressed file
}


impl Transfer {

    pub fn new() -> Self {
        Self {
            og_df: DataFrame::default(),          // dataframe after schema check
            compressed_df: DataFrame::default(),  // compresed dataframe; pre file write
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
        //! 1) Verify incoming schema is equal to reference value (ingestion.rs)
        //! 2) Pass through compression algos (compression.rs)
        //! 3) Write to new file
        
        let mut transfer = TransferIngestion::new();
        // validate incoming schema matches transfer dataset
        let schema_check = transfer.check_schema_validity(filepath).unwrap();
        
        // begin compression of dataset
        let mut block_compression = RLECompressedBlockNumberSeries::new();
        block_compression.block_number(schema_check);

       
        Ok(())
    }
}