use std::io;

fn main() {
    println!( "Fibonacci Generator" );
    let fibo_num = read_input();
    println!( "Generating Fibonacci number {}.", fibo_num );

    let result = if fibo_num == 1 || fibo_num == 2 {
        1
    } else {
        let mut result = 0;
        let mut i = 3;
        let mut current_fibo_minus_2 = 1;
        let mut current_fibo_minus_1 = 1;
        while i <= fibo_num {
            result = current_fibo_minus_2 + current_fibo_minus_1;
            current_fibo_minus_2 = current_fibo_minus_1;
            current_fibo_minus_1 = result;
            i += 1;
        }

        result
    };

    println!( "Result: {}", result );
}

fn read_input() -> u32 {
    println!( "Which Fibonacci number (1, 2, ...) do you want to see?" );
    loop {
        let mut fibo_num = String::new();
        io::stdin().read_line( &mut fibo_num )
            .expect( "Failed to read input." );

        let fibo_num: u32 = match fibo_num.trim().parse() {
            Ok( num ) => num,
            Err( _ ) => {
                println!( "Input an integer greater than zero." );
                continue
            }
        };

        if fibo_num < 1 {
            println!( "Input an integer greater than zero." );
            continue
        }

        // If we made it here, then we have a valid number.
        return fibo_num;
    }
}
