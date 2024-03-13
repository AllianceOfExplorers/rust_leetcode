fn main() {

}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
   for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32,j as i32];
            }
        }
   }

   return vec![];
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![0,1,3], 1),vec![0,1]);
    assert_eq!(two_sum(vec![2,7,11,15], 9),vec![0,1]);
}