use stdin::io;

fn main(){
    let cond = 10.1_f32 > 5.5_f32;
    println!("this statement is {}", cond);

    //user input
    println!("Enter your Age: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let age: i32 = input.trim().parse().unwrap();
    //if else statements
    if age <= 16 {
        println!("You are too young!");
        
    }

    else if age == 40 && age > 16 && age <= 30 {
        println!("You are obviously an Adult!");

    }

    else{
        println!("what age are you? ");
        
    }

    






}