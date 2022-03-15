fn main() {
    //hello world
    println!("Hello, world!");


    //variables and data types , mutable variables
    let x = 4;
    println!("Value of x is: {}",x);

    let mut y = 5;
    println!("value of y: {}",y);
    y = 10;
    println!("Reassigned Value of y: {}",y);

    let z: i32 = -30; //signed integer
    println!("z: {}",z);


    //if else statement
    let a = true;
    let b = false;
    
    if a {
        println!("a is true");
    }

    if b {
        println!("b is true");
    }


    //Composite types
    
    //tuples - return a set of different types of values
    //are immutable

    let tup = (5, "Hello", false);
    println!("The first value in the tuple is: {}",tup.0);
    println!("2nd : {} and 3rd : {}", tup.1,tup.2);
    println!("Tuple is - {:?}",tup);

    //destructuring
    // let tup1 = (1,2,3);
    // let (val1, val2, _) = a;
    // println!("2nd : {} and 3rd : {}", val1,val2);


    //arrays - set of same type of values
    let a = [1,2,3,4];
    println!(" Array 1st : {}", a[0]);

    let a = [0; 10]; //array with 0 repeated 10 times
    println!("Array : {:?}",a);

    let a: [i32; 10] = [2; 10];
    println!("Array : {:?}",a);


    //mutable arrays
    let mut a = [0];
    println!("Array 1st : {}",a[0]);
    
    a[0] = 4;
    println!("Array 1st updated : {}",a[0]);


    //control flow
    let num = 2;

    if num==1{
        println!("You won");
    }
    else if num == 2{
        println!("Try again");
    }
    else{
        println!("You lost");
    }


    //loops
    let mut x = 1;
    //loop
    loop{
        x = x*5;

        if x>5000 {
            break;
        }
        println!("x : {}",x);
    }

    println!("Out of loop"); 

    //while
    let mut x = 1;
    while x<5000 {
        x = x*5;
        println!("x : {}",x);
    }
    println!("out of while");

    for temp in 0..10{
        println!("temp : {}",temp);
    }

    for temp in 0..=10{
        println!("temppp : {}",temp);
    }

    let x = [4,1,3,6];
    for val in x{
        println!("val is {}",val);
    }

    //match statement - exhaustive pattern matching (switch and if)
    let x = 10;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        _ => println!("x is invalid")
    } 

    let x = true;
    let y = false;

    match (x,y) {
        (true,true) => println!("x and y both true"),
        (true,false) => println!("x : true and y : false"),
        (false,true) => println!("x : false and y : true"),
        //(false,false) => println!("x and y both false"),
        _ => println!("invalid for our use case") //catch all branch
    }

    //A macro in Rust is a like a function that takes a variable number of input arguments.
    //The todo! macro is used to identify unfinished code in the Rust program.
    //The macro is helpful for prototyping, or when you want to indicate behavior that isn't complete.

    //todo macro
    //todo!("Display message using println!() macro");

    //variable shadowing
    let n = 1;
    println!("{}",n);
    let n = 1+1;
    println!("{}",n);

    //isize usize 8 16 32(default) 64 128 f32 f64(default)
    let num:f32 = 5.0;
    println!("num : {}",num);
    
    println!("1 + 2 = {}", 1u32 + 2);

    let is_bigger = 1>4;
    println!("Is 1 > 4 ... {}",is_bigger);

    //char
    let s = 's';
    println!("char s : {}", s);

    //String type data aka string slice are text data that can change as your program runs.
    //The &str references are immutable views into the text data that don't change as your program runs.
    //For now, you can think of &str as a pointer to immutable string data. String literals are all of type &str.
    
    let char1: char = 'A';
    let char2 = 'B';
    let string1 = "C";
    let string2: &str = "D";

    println!("{},{},{},{}",char1,char2,string1,string2);
    println!("hard back");

    //tuples
    let tup = ('E',5i32, true);
    println!("tuple : {:?}",tup);
    //type signature of this tuple is (char,i32,bool)

    //structs
    //classic structs-- similar to C structs where fields have name
    struct Student{name:String, level:u8, remote:bool}
    

    //tuple structs-- fields dont have name to access fields we have to specify the index like we do in tuples
    struct Grades(char,char,char,f32);


    //unit structs-- most commonly used as markers (used in Rust traits)
    //struct Unit;

    //struct types are defined outside of the main function
    //instantiating a struct
    
    let user1 = Student{name:String::from("kk menon"), remote:true, level:2};
    
    let mark1 = Grades('A','A','B',9.95);

    println!("Name: {} \n Level:{} \n Remote:{} \n Grades: {},{},{}, Average:{}",user1.name,user1.level,user1.remote,mark1.0,mark1.1,mark1.2,mark1.3);

    //String data that's stored inside another data structure, such as a struct or vector, must be converted from a 
    //string literal reference (&str) to a String type. To do the conversion, we use the standard String::from(&str) method. or .to_string()

    //Enums
    // enum WebEvent{
    //     WELoad,
    //     WEKeys(String,char),
    //     WEClick{x:i64,y:i64}
    // }
    //weload has no associated data 
    //wekeys has 2 fields
    //weclick contains an anonymous struct with named fields


    //Enums
    //up,down..etc are variants of the Direction enum
    enum Direction{
        Up,
        Down,
        Left,
        Right
    }

    let player_direction:Direction = Direction::Up;

    match player_direction{
        Direction::Up => println!("\nPlayer moving up"),
        Direction::Down => println!("\nPlayer moving Down"),
        Direction::Left => println!("\nPlayer moving Left"),
        Direction::Right => println!("\nPlayer moving Right")
    }

    //define an enum with structs
    #[derive(Debug)]
    struct KeyPress(String,char);

    #[derive(Debug)]
    struct MouseClick{x:i64,y:i64}

    #[derive(Debug)]
    enum WebEvent{
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress)
    }

    //instantiating enum variants

    let we_load:WebEvent = WebEvent::WELoad(true);

    let click = MouseClick{x:100,y:200};
    let we_click:WebEvent = WebEvent::WEClick(click);

    let keys = KeyPress(String::from("Ctrl+"),'N');
    let we_keys:WebEvent = WebEvent::WEKeys(keys);
    
    println!("\nWeb Event enum structure {:#?} \n\n {:#?} \n\n {:#?} \n",we_load,we_click,we_keys);
    //#[derive(Debug)] lets us see certain values during the code execution that aren't viewable in standard output
    //to view debug data in println! we use {:#?} to format data in a readable manner

    //functions
    let formal = "Good Bye";
    goodbye(formal);

    println!("500 Divided by 10 Returns {}",divide_by_ten(500));
    println!("500 Divided by 5 Returns {}",divide_by_5(500));

}

fn goodbye(msg:&str){
    println!("\n{}",msg);
}

//last line indicates value to return
fn divide_by_ten(num:u32)->u32{
    num/10
}

fn divide_by_5(num:u32)->u32{
    if(num==0){
        return 0;
    }
    num/5
}
