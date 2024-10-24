use std::num::ParseIntError;

fn main() {
    let result = square("RT");  
    println!("{:?}", result);
}


fn square(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n * n),
        Err(e) => Err(e),
    }
}
