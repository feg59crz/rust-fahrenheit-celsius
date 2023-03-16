use std::io;

fn fah_to_c(fah :f32) -> f32 {
    ( fah - 32.0 ) * 5.0/9.0
}

fn c_to_fah(c :f32) -> f32 {
    ( c * 9.0/5.0 ) + 32.0
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

        if conversion < 1 || conversion > 2 {
            println!("\nInvalid option!");
            continue;
        }
        
        let mut value: String = String::new();

        println!("\nPlease input the value to conversion: ");

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("\nerror: {err}");
                continue;
            }
        };
        let unit: String;

        let result: f32 = if conversion == 1 {
            println!("\nConverting {value} Fahrenheit to Celsius...");
            unit = "ºC".to_string();
            fah_to_c(value) 
        } else {
            println!("\nConverting {value} Celsius to Fahrenheit...");
            unit = "ºF".to_string();
            c_to_fah(value) 
        };
        println!("\nConversion Result: {result}{unit}");
        break;
    }
    
}
