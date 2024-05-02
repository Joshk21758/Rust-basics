fn test_one(x: i32, y: i32) -> i32 {
    x + y

}

fn mult_num(a: f64, b: f64) -> f64 {
    a * b

}

fn main(){
    println!("the sum is: ");
    let sum = test_one(10, 10);
    println!("{}", sum);
    println!("the product is: ");
    let pro = mult_num(5.5, 6.7);
    println!("{}", pro);
    

}