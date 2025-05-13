use std::mem;
use polars::prelude::*;
use owo_colors::OwoColorize;

pub struct RLECompressedBlockNumberSeries {
    pub values: Vec<u32>,    // Unique values in sequence
    pub counts: Vec<u16>,    // Count of consecutive repetitions
}

impl RLECompressedBlockNumberSeries {

    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            counts: Vec::new(),
        }
    }

    pub fn compress_block_number(&mut self, dataset: &DataFrame) -> Result<(Vec<u32>, Vec<u16>), Box<dyn std::error::Error>> {
        // establish incoming col len // let num_rows = dataset.height();

        let blocks = dataset.column("block_number").unwrap();
        let block_vec: Vec<Option<u32>> = blocks.u32()?.into_iter().collect();

        // early return if vec is empty
        if block_vec.is_empty() {
            return Ok((Vec::new(), Vec::new()));
        }

       // set initial block as current value, and initial count as 1
        let mut current_value = block_vec[0].unwrap();
        let mut current_count = 1 as u16;

        // iterate through blocks, skip first, 
        for block in block_vec.iter().skip(1) {
            let b = block.unwrap();
            if b == current_value {
                current_count += 1 as u16;
            } else {
                self.values.push(current_value);
                self.counts.push(current_count.into());

                current_value = b;
                current_count = 1;
            }
        }
        self.values.push(current_value);
        self.counts.push(current_count.into());

        // check size comparisons
        let block_size = block_vec.capacity() * mem::size_of::<Option<u32>>();
        let compressed_size = self.values.capacity() * mem::size_of::<u32>() + 
                            self.counts.capacity() * mem::size_of::<u16>();

        println!("Original block index: {} bytes", block_size.red());
        println!("Compressed block index: {} bytes", compressed_size.green());
        println!("Compression Ratio: {:.2}", block_size as f64 / compressed_size as f64);

        // assert that output is equal in len to input
        // assert_eq!()
        Ok((self.values.clone(), self.counts.clone()))

    }
}



