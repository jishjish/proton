
use polars::prelude::*;


pub struct Transfer {
    pub og_df: DataFrame,
    pub compressed_df: DataFrame,
}



impl Transfer {

    pub fn new() -> Self {
        Self {
            og_df: DataFrame::default(),          // dataframe after schema check
            compressed_df: DataFrame::default(),  // compresed dataframe; pre file write
        }
    }
}