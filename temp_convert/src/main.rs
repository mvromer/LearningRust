use std::io;

enum ConvertMethod {
    FahrenheitToCelsius,
    CelsiusToFahrenheit
}

fn main() {
    println!( "Temperature Converter" );
    let convert_method = get_convert_method();
    let input_temp = get_input_temperature( &convert_method );

    let (output_temp, input_units, output_units) = match convert_method {
        ConvertMethod::FahrenheitToCelsius => ((input_temp - 32.0) / 1.8, "°F", "°C"),
        ConvertMethod::CelsiusToFahrenheit => (input_temp * 1.8 + 32.0, "°C", "°F")
    };

    println!( "{}{} = {}{}", input_temp, input_units, output_temp, output_units );
}

fn get_convert_method() -> ConvertMethod {
    loop {
        println!( "Which temperature conversion do you want to do?" );
        println!( "1. Fahrenheit to Celsius" );
        println!( "2. Celsius to Fahrenheit" );

        let mut convert_method = String::new();
        io::stdin().read_line( &mut convert_method )
            .expect( "Failed to read input." );

        let convert_method = match convert_method.trim().parse() {
            Ok( 1 ) => ConvertMethod::FahrenheitToCelsius,
            Ok( 2 ) => ConvertMethod::CelsiusToFahrenheit,
            _ => {
                println!( "Invalid option given." );
                continue
            }
        };

        return convert_method;
    }
}

fn get_input_temperature( convert_method: &ConvertMethod ) -> f64 {
    let source_units = match convert_method {
        ConvertMethod::FahrenheitToCelsius => "Fahrenheit",
        ConvertMethod::CelsiusToFahrenheit => "Celsius"
    };

    loop {
        println!( "What temperature in {} do you want to convert?", source_units );

        let mut input_temperature = String::new();
        io::stdin().read_line( &mut input_temperature )
            .expect( "Failed to read input temperature." );

        let input_temperature: f64 = match input_temperature.trim().parse() {
            Ok( num ) => num,
            Err( _ ) => {
                println!( "Invalid input temperature given." );
                continue
            }
        };

        return input_temperature;
    }
}
