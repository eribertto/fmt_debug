use std::u8;

// all types which want to use std::fmt formatting traits require implementation to be printable
// automatic implementations are only provided for types such as in the std library. All others must
// be manually implemented somehow

#[allow(dead_code)]
struct UnPrintable(i32);
#[derive(Debug)]
struct DebugPrintable(i32);

// derive the fmt::Debug implementation for Structure which contains a single i32
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// the derive attribute automatically creates the implementation
fn main() {
    // printing with {:?} is similar to with {}
    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // Structure is now printable
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will also print!", Deep(Structure(9)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

	let name = "Peter";
	let age = 27;
	let peter = Person{ name, age };

	// pretty print
	println!("{:?}", peter);
	println!("Hello my name is {} and I'm {} years old", peter.name, peter.age);
}
