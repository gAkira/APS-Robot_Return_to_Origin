mod robot_return_to_origin;

fn main() {
    let s = "RLDUUDRLLRUDURLDURLDDRUL".to_string();
    println!("{}", robot_return_to_origin::judge_circle(s));
}
