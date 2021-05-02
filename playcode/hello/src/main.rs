fn main() {
    // let mut x = 123;
    // x = 444;
    // {
    //     let y = 555;
    //     println!("{} plus {}", x, y)
    // }
    // println!("{}", x);
    // println!("Hello, world!");

    // -----------------------------------------------------------------------------
    // enums

    //enum in rust is kinda like union in C - can return any data type
    // enum FirstEnum {
    //     Emptyz,
    //     Ammo(i32),
    //     Place { x: i32, y: i32 },
    // }

    //import everything from inside our enum
    // use FirstEnum::*;
    // let item = Place { x: 33, y: 22 };
    // println!("{}", item.x);

    // ----------------------- Option

    //option is a "generic" enum that you will use all the time
    //represents when something is absent or present
    //it is so popular that we don't need a "use" statement to bring in Some() or None. They're already included!
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // this is how you define an option - we specify the type that Some() will wrap
    // let my_var: Option<i32> = Some(3);
    // let my_var: Option<i32> = None;

    //but in fact we can actually leave off the declaration and compiler will infer itself
    let my_var = Some(3); //doesn't seem to work with None

    //handy helper method that returns true if x is the Some variant
    if my_var.is_some() {
        println!("is indeed some!");
    }

    //and the opposite
    if my_var.is_none() {
        println!("oh no im none!")
    }

    //we can also treat an option as a Vector of 0 or 1 items
    for i in my_var {
        println!("i is {}", i);
    }

    //we use "patterns" to examine enums
    //check for a single pattern
    if let Some(x) = my_var {
        println!("my_var is indeed equal to {}", x);
    }
    //check all patterns (MUST be exhaustive)
    match my_var {
        Some(x) => {
            println!("my_var is indeed equal to {}", x);
        },
        None => {
            println!("my_var is none");
        }
    }

    // ----------------------- Result
    //defn:

    // #[must_use] //compiler warning to "silently drop a result". Basically forces you to handle errors instead of ignoring them
    // enum Result<T,E> {
    //     Ok(T),
    //     Err(E),
    // }
    // also don't need to define, so commenting out

    use std::fs::File;

    let result = File::open("Cargo.lock");
    if result.is_ok() { //also has helper methods
        println!("result is ok!")
    } else {
        println!("result failed!")
    }


}