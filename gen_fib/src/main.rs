use std::io;

fn main() {
    println!( "Fibonacci Generator" );
    let fibo_num = read_input();
    println!( "Generating Fibonacci number {}.", fibo_num );
}

fn read_input() -> u32 {
    println!( "Which Fibonacci number do you want to see?" );
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
