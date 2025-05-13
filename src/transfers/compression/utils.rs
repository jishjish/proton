// use std::collections::HashMap;

// /// get the mode of a vector; return the occurrence count NOT mode val
// pub fn mode(sequence: &[Option<u32>]) -> u32 {

//     let mut modes: HashMap<u32, u32> = HashMap::new();

//     for i in sequence.iter() {
//         let val = i.unwrap();

//         if modes.contains_key(&val) {
//             let current = modes.get(&val);
//             let cu = current.unwrap();
//             modes.insert(val, cu + 1);
//         } else {
//             modes.insert(val, 1);
//         }
//     }
    
//     let (_mode_key, mode_count) = modes.iter()
//         .max_by_key(|&(_, count)| count)
//         .unwrap();
    
//     // println!("count: {}", mode_count);
//     *mode_count
// }






/// RLE based dataframe creation
// pub fn create_rle_dataframe() -> DataFrame {
pub fn create_rle_dataframe() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}