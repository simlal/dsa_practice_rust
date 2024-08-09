
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut num_duplicates = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if i != j && nums[i] == nums[j] {
                    // println!("{} is equal to {}", nums[i], nums[j]);
                    num_duplicates += 1;
                }
            }
        }
        num_duplicates
    }
}
pub fn run() {
    println!("Challenge file name: {}\n", file!());
    println!("########################  SOLUTION-DEBUGGING-START  ########################\n");
    let mut my_vec = vec![1, 2, 3, 4, 2];
    let result = Solution::remove_duplicates(&mut my_vec);
    println!("Number of duplicates is: {}", result);
    println!("\n#######################  SOLUTION-DEBUGGING-END    ########################");
}