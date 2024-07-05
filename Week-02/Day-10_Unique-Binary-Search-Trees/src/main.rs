use std::collections::HashMap;

use app::get_array;

#[derive(Debug)]
pub struct Branch {
    pub father: Option<u8>,
    pub left: Option<Box<Branch>>,
    pub right: Option<Box<Branch>>,
}



fn main() {
    // let tree1:Branch = Branch { 
    //         father: Some(8), 
    //         left: Some(Box::new(Branch {
    //             father: Some(1), 
    //             left: None, 
    //             right: Some(Box::new(Branch{
    //                 father:Some(5), 
    //                 left: None, 
    //                 right: None
    //             }))
    //         })), 
    //         right: Some(Box::new(Branch {
    //             father: Some(1), 
    //             left: None, 
    //             right: None 
    //         }))
    //     };
    // dbg!(tree1);

    let numbers = get_array(4);
    // dbg!(numbers.clone());

    let mut trees: HashMap<u8, Branch> = HashMap::new();
    for root in &numbers {
        // Make numbers copy.
        let mut rest = numbers.clone();
        // Get index to remove.
        let index = (*root - 1) as usize; 
        rest.remove(index);
        // println!("{:?}",rest);
        let tree = Branch{
            father: Some( *root ),
            left: None,
            right: None
        };
        trees.insert(*root, tree);


        // while rest.len() > 0 {
        //     rest.remove(1);
        //     println!("{:?}",rest);
        // }
    }

    dbg!(trees);
    // println!("Trees: {:?}", trees);
}
