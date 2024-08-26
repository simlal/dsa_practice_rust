struct Solution;

impl Solution {
    pub fn my_func(arg1: &str) -> () {
        println!("arg1: {}", arg1);
    }
}
pub fn run() {
    println!("Challenge file name: {}\n", file!());
    println!("########################  SOLUTION-DEBUGGING-START  ########################\n");
    
    Solution::my_func("test");

    println!("\n#######################  SOLUTION-DEBUGGING-END    ########################");
}