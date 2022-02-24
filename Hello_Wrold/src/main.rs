fn main() {
    println!("Bismillah");

    println!("Hello, world!");

    println!("Hello rust, I'm fahim faisaal");

    // online comments
    /* 
        block comments
        haha
    */

    // Doc comments which are parsed into HTML library documentation:
    // Generate libraries docs for the following documentation
    // //! Generate libraries docs for the enclosing item.

    // This is an example of a line comment
    // There are two slashes at the beginning of the line
    // And nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /* 
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But
     * block comments are extremely useful for temporarily disabling
     * chunks of code. /* Block comments can be /* nested, */ */
     * so it takes only a few keystrokes to comment out everything
     * in this main() function. /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    /*
    In general, the `{}` will be automatically replaced with any arguments. These will be stringified and it's receive positional arguments
    */
    println!("{} is Ten", 10i8); // Without a suffix, 10 becomes an i32. You can change what type 10 is to i8

    // There are various optional patterns this works with. Positional arguments can be used.
    println!("**{0}** is the first argument and **{1}** is the second argument", "Fahim", "Faisal");

    // As a named arguments
    println!("{first_name} {age} {last_name}", 
        first_name = "Fahim",
        last_name = "Faisal",
        age = 23i8
    );

    // Special formatting can be specified after a `:`. 

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    // the second positional argument converted to binary

     // You can right-align text with a specified width. This will output
    // "000001". 5 white spaces and a "1".
    println!("{number:0>width$}", number=5i8, width=6); // width - number.len()

    let str = format!("My name is {0}, {1} {0}", "Bond", "James");
    println!("{}", str);
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[derive(Debug)] // here derive is an attribute
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);
    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{:?}` can be printed now", Deep(Structure(10)));
    // FIXME ^ Comment out this line.

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: i8
    }

    let fahim: Person = Person { name: "fahim faisal", age: 23 };

    println!("{} {}", fahim.name, fahim.age);

    // A unit struct
    struct Unit;
    
    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[allow(dead_code)]
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    // Instantiate a Point
    let point: Point = Point { x: 10.98, y: 5.4 };

    let top_left = Point { y: 3.4, ..point };
    let bottom_right = Point { x: 11.20, ..point };

    let rect = Rectangle { top_left, bottom_right };
    
    //* Rust also provides "pretty printing" with {:#?}
    println!("{:#?}", rect.top_left);
    println!("{:#?}", rect.bottom_right);

    // destructuring form struct
    let Point { x: left_angle, y: right_angle }  = point;

    println!("x: {} y: {}", left_angle, right_angle);

    // instance of Pair
    let pair = Pair(100, 10_000.01);

    println!("first: {:b} second: {}", pair.0, pair.1);
}
