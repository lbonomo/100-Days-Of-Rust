use serde::Deserialize;
use std::collections::HashMap;



#[derive(Deserialize, Debug)]
pub struct Tree {
    pub root: u8,
    pub levels: Option<Vec<Level>>,
}

#[derive(Deserialize, Debug)]
pub struct Level {
    pub level: u8,
    pub left: Option<u8>,
    pub right: Option<u8>,
}

fn make_level(father: u8, level:u8, nums:Vec<u8>) {
    println!("{:?}", nums);
}


fn main() {
    #![feature(extract_if)]
    
    const INPUT:u8 = 3;

    // let mut trees:HashMap<u8, Tree> = HashMap::new();
    
    // Make a array of `u8` from 1 to INPUT.
    // Ex: INPUT = 3 -> [1, 2, 3]
    let numbers:[u8; INPUT as usize] =  (1..=INPUT).collect::<Vec<_>>().try_into().expect("wrong size iterator");

    // println!("{:?}", numbers);

    for n in numbers {
        let id = n;
        let level_number = 0;
        
        println!("{}", n);



        let mut nums = numbers.to_vec();
        // nums.remove(id as usize);
        nums.extract_if(|x| x == &n );
        
        println!("{:?}", nums);

        // make_level(n, level_number, nums);
        // if level_number == 0 {
        //     // Level 0 / Root.
        //     let level = make_level(n, level_number, nums);
        //     let tree:Tree = Tree {
        //         root:n,
        //         levels: None,
        //     };
        // }
        

        
        // trees.insert(id, tree);
    }

    
    // println!("{:?}", trees);
}
