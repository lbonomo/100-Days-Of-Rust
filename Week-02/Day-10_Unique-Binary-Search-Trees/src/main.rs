use std::ptr::null;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Tree {
    pub root: Option<u8>,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

// #[derive(Deserialize, Debug)]
// pub struct Branch {
//     pub level: u8,
//     pub left: Option<Box<Branch>>,
//     pub right: Option<Box<Branch>>,
// }


// fn make_level(father: u8, level:u8, nums:Vec<u8>) {
//     println!("{:?}", nums);
// }


fn main() {
   
    // const INPUT:u8 = 3;
    // let mut trees:HashMap<u8, Tree> = HashMap::new();

    let tree1:Tree = Tree { 
            root: Some(8), 
            left: Some(Box::new(Tree {
                root: Some(1), 
                left: Some(4), 
                right: Some(5)
            })), 
            right: Some(Tree {
                level: 1, 
                left: Box::new(10), 
                right: Some(None) 
            })
        };
    dbg!(tree1);


    // print!("{:?}", tree1);

    // Make a array of `u8` from 1 to INPUT.
    // Ex: INPUT = 3 -> [1, 2, 3]
    // let numbers:[u8; INPUT as usize] =  (1..=INPUT).collect::<Vec<_>>().try_into().expect("wrong size iterator");

    // println!("{:?}", numbers);

    // for n in numbers {
    //     let id = n;
    //     let level_number = 0;
        
    //     // println!("{}", n);



    //     let mut nums = numbers.to_vec();
    //     // nums.remove(id as usize);
    //     // nums.extract_if(|x| x == &n );
        
    //     // println!("{:?}", nums);

    //     // make_level(n, level_number, nums);
    //     // if level_number == 0 {
    //     //     // Level 0 / Root.
    //     //     let level = make_level(n, level_number, nums);
    //     //     let tree:Tree = Tree {
    //     //         root:n,
    //     //         levels: None,
    //     //     };
    //     // }
        

        
    //     // trees.insert(id, tree);
    // }

    
    // println!("{:?}", trees);
}
