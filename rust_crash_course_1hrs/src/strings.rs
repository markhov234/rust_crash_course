// // Primitive str = Immutable fixed-length strinf somwhere in memory
// // String = Growable, heap-allocated data strucutes - Use when you need to modify or own string data

// pub fn run() {
//     let mut hello = String::from("Hello ");

//     println!("{}", hello);
//     // Length string
//     println!("{}", hello.len());

//     // Push on a CHAR
//     // hello.push('W');
//     // Push on a STRING
//     hello.push_str("World ! ");

//     // Capacity in bytes
//     println!("Capacity : {}", hello.capacity());
//     // Check if EMPTY
//     println!("Is Empty : {}", hello.is_empty());
//     // Contains
//     println!("Contains 'World' {}", hello.contains("World"));
//     // Replace
//     println!("Replace World : {}", hello.replace("World", "there"));
//     // Loop through string by whitespace
//     for word in hello.split_whitespace() {
//         println!("Loop is : {}", word)
//     }

//     // Create a string with capacity
//     let mut s = String::with_capacity(10);

//     s.push('a');
//     s.push('b');

//     // Assertion Testing -- Fait pour tester les strings
//     assert_eq!(2, s.len());
//     assert_eq!(10, s.capacity());

//     println!("{}", s);
// }
