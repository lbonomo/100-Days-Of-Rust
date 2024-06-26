use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Trees {
    pub id: u8,
    pub tree: Tree,
}

#[derive(Deserialize, Debug)]
pub struct Tree {
    pub root: i8,
    pub levels: Level,
}

#[derive(Deserialize, Debug)]
pub struct Level {
    pub level: u8,
    pub left: u8,
    pub right: u8,
}


fn main() {
    // let trees:Trees= todo!();

    const INPUT:u8= 3;
    
    // let num:[u8; INPUT as usize]
    let mut num:Vec<u8> = Vec::new();

    for n in 1..INPUT+1 {
        num.push(n);    
    } 
    
    
    println!("{:?}", num);
}
