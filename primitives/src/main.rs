use std::mem;

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1_u8, 200_u16, 2u8), (4_u8, -1_i8), -2_i8);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    println!("reverse pair {:?}", reverse((1, true)));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (integer, string, float, boolean) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", integer, string, float, boolean);

    let my_first_array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("This is the first array {:?}", my_first_array);

    // all elements in fill with 5
    let fill_array: [i32; 100] = [0; 100];
    println!("this is sequence array {:?}", fill_array);

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&my_first_array));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&fill_array);
    let len = my_first_array.len();
    analyze_slice(&my_first_array[2..len - 1]);
}
