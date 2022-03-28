use std::env;
  fn main(){
    let args: Vec<String> = env::args().collect();
    let correct: f64 = args[1].parse().unwrap();
    let total: f64   = args[2].parse().unwrap();
    let precent: i32 = convert((correct/total)*100.0);
    let letter_grade = match precent{
        98..=100 => "A+",
        93..=97  => "A",
        90..=92  => "A-",
        87..=89  => "B+",
        83..=86  => "B",
        80..=82  => "B-",
        77..=79  => "C+",
        73..=76  => "C",
        70..=72  => "C-",
        67..=69  => "D+",
        63..=66  => "D",
        60..=62  => "D-",
        _ => "F"
    };
    println!("you scored a {}%, thats an {}", precent, letter_grade);
}
fn convert(x: f64) -> i32 {
    x.round().rem_euclid(2f64.powi(32)) as u32 as i32
}