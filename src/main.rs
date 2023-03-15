use std::io;


fn fah_to_c(fah :i32) -> i32 {
    fah
}


fn main() {
    
    loop {
        let mut conversion: String = String::new();
        println!("\n=== Fahrenheit - Celsius conversion ===\n");
        println!("Please input the conversion type.\n");
        println!("1 - Fahrenheit to Celsius");
        println!("2 - Celsius to Fahrenheit\n");
        println!("Input your choose (1 or 2): ");
        
        io::stdin()
            .read_line(&mut conversion)
            .expect("Failed to read line.");
        
        let conversion: u8 = match conversion.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("\nerror: {err}");
                continue;
            }
        };

        if (conversion < 1 || conversion > 2) {
            println!("\nInvalid option!");
            continue;
        }
    }
    
}
