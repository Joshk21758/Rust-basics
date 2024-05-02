use stdin::io;

fn main(){
    //request user input
    println!("Enter a Number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    
    //converting input str to int
    let int_inp: i32 = input.trim().parse().unwrap();
    println!("{}", int_inp + 10);


    

}