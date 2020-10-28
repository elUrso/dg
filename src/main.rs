use std::env;

enum Mode {
    Celsius, Fahrenheit, Kelvin
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("format invalid, please use % dg N[cfk]");
    }

    let mut input = args[1].clone();
    
    let mode = match input.pop() {
        Some('c') => Mode::Celsius,
        Some('f') => Mode::Fahrenheit,
        Some('k') => Mode::Kelvin,
        Some(_) | None => panic!("temp must end in c, f or k"),
    };

    let num: f64 = input.parse().expect("err parsing number");

    let (c, f, k): (f64, f64, f64);

    match mode {
        Mode::Celsius => {
            c = num;
            k = c + 273.0;
            f = 1.8 * c + 32.0;
        }
        Mode::Fahrenheit => {
            f = num;
            c = (f - 32.0) / 1.8;
            k = c + 273.0;
            
        }
        Mode::Kelvin => {
            k = num;
            c = k - 273.0;
            f = 1.8 * c + 32.0;
        }
    }

    println!("{:.2}c {:.2}f {:.2}k", c, f, k);
}
