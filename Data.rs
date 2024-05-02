fn main(){
    //tuples
    let mut tup1 = ("Josh", 'd', 20, 35.5, true);
    tup1.1 = 'A';
    println!("This is a tuple: {}", tup1.4);

    //Arrays
    let mut arr = [10, 20, 30, 40, 50];
    arr[0] = 60;
    arr[1] = 70;
    println!("This is the number: {}", arr[4]);
    

}