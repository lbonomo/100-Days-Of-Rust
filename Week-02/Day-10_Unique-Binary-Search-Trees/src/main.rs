#[derive(Debug)]
pub struct Branch {
    pub father: Option<u8>,
    pub left: Option<Box<Branch>>,
    pub right: Option<Box<Branch>>,
}

fn main() {
    let tree1:Branch = Branch { 
            father: Some(8), 
            left: Some(Box::new(Branch {
                father: Some(1), 
                left: None, 
                right: Some(Box::new(Branch{
                    father:Some(5), 
                    left: None, 
                    right: None
                }))
            })), 
            right: Some(Box::new(Branch {
                father: Some(1), 
                left: None, 
                right: None 
            }))
        };
    dbg!(tree1);
}
