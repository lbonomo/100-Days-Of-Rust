pub fn get_array(number: u8) -> Vec<u8> {
    let numbers:Vec<u8> =  (1..=number).collect::<Vec<_>>().try_into().expect("wrong size iterator");
    numbers
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
