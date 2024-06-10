pub fn merge(nums1:Vec<u8>, nums2:Vec<u8>) -> Vec<u8> {
    let mut result:Vec<u8>  = vec![];
    
    for num in nums1.iter() {
        println!("{}", num );
        result.push(num);
        // if num > 0 {
        // }
    };

    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums1 = vec![1,2,3,0,0,0];
        let nums2 = vec![2,5,6];
        let output = vec![1,2,2,3,5,6];

        let result = merge(nums1, nums2);
        assert_eq!(result, output);
    }
}
