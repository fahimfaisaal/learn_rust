enum WebEvent {
    // enum props like unit type
    PageLoad,
    PageUpload,
    // enum props can be line tuple type
    KeyPress(char, i8),
    Paste(String),
    // enum props can be struct type
    Click { x: i64, y: i64 }
}

enum Calculation {
    Addition,
    Subtraction,
    Division,
    Multiplication
}

// create a type alias
type Operations = Calculation;

impl Calculation {
    fn run(&self, x: i64, y: i64) -> i64 {
        match self {
            Self::Addition => x + y,
            Self::Subtraction => x - y,
            Self::Division => x / y,
            Self::Multiplication => x * y
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUpload => println!("Page Uploaded"),
        // Destructure `letter and number` from inside the `enum`.
        WebEvent::KeyPress(letter, number) => println!("Pressed {} {}.", letter, number),
        WebEvent::Paste(str) => println!("Pasted {}", str),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("Clicked at x = {} and y = {}", x, y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('A', 10);
     // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("Allahu Akbar".to_owned());
    let clicked = WebEvent::Click { x: 10, y: -80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUpload;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(load);
    inspect(unload);
}