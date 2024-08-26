
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                // println!("nums[{}]={}", i, nums[i]);
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }
}
pub fn run() {
    println!("Challenge file name: {}\n", file!());
    println!("########################  SOLUTION-DEBUGGING-START  ########################\n");
    
    let mut vec1 = vec![3, 2, 2, 3];
    let my_val = 3;
    println!("nums: {:?}", vec1);
    println!("val: {}\n", my_val);

    let result = Solution::remove_element(&mut vec1, my_val);
    println!("result = {}", result);
    println!("after_nums: {:?}", vec1);


    println!("\n#######################  SOLUTION-DEBUGGING-END    ########################");
}