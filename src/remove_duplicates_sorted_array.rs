
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut left = 1;
        for right in 1..nums.len() {
            if nums[right] != nums[right - 1] {
                nums[left] = nums[right];
                left += 1;
            }
        }

        left as i32
    }
}
pub fn run() {
    println!("Challenge file name: {}\n", file!());
    println!("########################  SOLUTION-DEBUGGING-START  ########################\n");
    
    // let mut my_vec = vec![0,0,1,1,1,2,2,3,3,4];
    let mut vec2 = vec![1,1,2];
    let result = Solution::remove_duplicates(&mut vec2);
    println!("Number of duplicates is: {}", result);
    println!("Mutated vec: {:?}", vec2);
    
    println!("\n#######################  SOLUTION-DEBUGGING-END    ########################");
}