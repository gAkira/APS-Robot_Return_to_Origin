// Use module created (file robot_return_to_origin.rs)
mod robot_return_to_origin;

fn main() {
    // Sequence of movements to the robot
    let s = "RLDULLLLLUUUUULLLLLUUUUURDRDRDRDRDDDDDDRRRRRUDLR".to_string();
    
    // Result of the given sequence
    println!("{}", robot_return_to_origin::judge_circle(s));
}
