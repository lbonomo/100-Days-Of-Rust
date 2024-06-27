use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct Trees {
//     pub id: u8,
//     pub tree: Tree,
// }

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


fn main() {
    
    const INPUT:u8= 3;
    let mut trees:Vec<Tree> = Vec::new();
    
    for n in 1..INPUT+1 {
        let data:Tree = Tree {
            root:n,
            levels: None,
        };
        trees.push(data);
    }
    
    
    
    
    println!("{:?}", trees);
}
