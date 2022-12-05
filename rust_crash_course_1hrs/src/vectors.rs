// Vectors are re-sizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value (REMOVE)
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value : {}", numbers[0]);

    //  Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectore are stack allocated
    println!(
        "Vector occupied : {} bytes",
        std::mem::size_of_val(&numbers)
    );

    // Get Slice
    let slice = &numbers;
    println!("Slice: {:?}", slice);

    // Loop through vector value
    for x in numbers.iter() {
        println!("Numbers: {}", x)
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
