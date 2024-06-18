pub fn method_01(nums1:Vec<u8>, nums2:Vec<u8>) -> Vec<u8> {
    let mut result:Vec<u8>  = nums1[0..nums2.len()].to_vec(); 

    result.extend_from_slice(&nums2);
    result.sort();
    return result
}

pub fn method_02(m:Vec<u8>,n:Vec<u8>) -> Vec<u8> {
    let mut result:Vec<u8> = [&m[..n.len()], &n].concat();
    result.sort();
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

        let result = method_01(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums1 = vec![1,2,3,0,0,0];
        let nums2 = vec![2,5,6];
        let output = vec![1,2,2,3,5,6];

        let result = method_02(nums1, nums2);
        assert_eq!(result, output);
    }
}
