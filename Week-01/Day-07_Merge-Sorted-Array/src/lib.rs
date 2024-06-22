pub fn method_01(nums1:Vec<i8>, nums2:Vec<i8>) -> Vec<i8> {
    let size = nums1.len() - nums2.len(); 
    let mut result:Vec<i8>  = nums1[0..size].to_vec(); 

    result.extend_from_slice(&nums2);
    result.sort();
    return result
}

pub fn method_02(m:Vec<i8>,n:Vec<i8>) -> Vec<i8> {
    let size = m.len() - n.len();
    let mut result:Vec<i8> = [&m[..size], &n].concat();
    result.sort();
    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
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

    #[test]
    fn it_works3() {
        let nums1 = vec![1,2,3,0,0,0,0];
        let nums2 = vec![-2,5,6,7];
        let output = vec![-2,1,2,3,5,6,7];

        let result = method_01(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let nums1 = vec![1,2,3,0,0,0,0];
        let nums2 = vec![-2,5,6,7];
        let output = vec![-2,1,2,3,5,6,7];

        let result = method_02(nums1, nums2);
        assert_eq!(result, output);
    }
}
