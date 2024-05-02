fn main(){
    struct Student{
        name: String,
        age: i32,
        sch: String,

    };

    struct Food{
        color: String,
        sze: f32,
        nme: String,

    };

    let mut a = Student{
        name: "Joshua",
        age: 21,
        sch: "Arakan",

    };

    let mut b = Food{
        color: "Red",
        sze: 5.5,
        nme: "Apple",
        
    };

    println!("{}", b.color);
    
}