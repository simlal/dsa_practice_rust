use std::vec;

struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; nums.len() * 2];
        for i in 0..nums.len() {
            ans[i] = nums[i];
            ans[i + nums.len()] = nums[i];
        }
        ans
    }
}
pub fn run() {
    println!("Challenge file name: {}\n", file!());
    println!("########################  SOLUTION-DEBUGGING-START  ########################\n");
    
    let my_vec = vec![1, 2, 1];
    let mut ans = Solution::get_concatenation(my_vec);

    println!("ans: {:?}", ans);

    println!("\n#######################  SOLUTION-DEBUGGING-END    ########################");
}