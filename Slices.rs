fn main(){
    //create array
    let mut my_arr = [10, 20, 30, 40, 50, 60];
    my_arr[0] = 12;

    //Slice
    let my_sli = &my_arr[1..4];
    println!("This is a slice {}", my_sli[2]);
    

}