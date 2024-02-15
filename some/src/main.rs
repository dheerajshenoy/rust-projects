use std::{io};

fn main() {
    let mut n : String = String::new();
    let mut arr: Vec<f32> = Vec::new();

    println!("Enter the number of elements: ");
    io::stdin()
        .read_line(&mut n)
        .expect("Could not read from STDIN");
    println!("Enter {} elements: ", n);

    let mut n: u8 = n.trim().parse().expect("Error Occured");
    
    let mut i = 0;
    while i < n {
        let mut num: String = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Could not read input");

        let num: Result<f32, _> = num.trim().parse();
        match num {
            Ok(num) => arr.push(num),
            Err(e) => println!("Error Occured: {}", e),
        }
        
        i += 1;
    }

    println!("Sum is {}", arr.iter().sum::<f32>());
}
