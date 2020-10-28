use std::env;

enum Temp {
    Celsius(f64), 
    Fahrenheit(f64), 
    Kelvin(f64)
}

impl Temp {
    fn show_all(&self) -> String {
        let (c, f, k): (f64, f64, f64);

        match self {
            Temp::Celsius(temp) => {
                c = *temp;
                k = c + 273.0;
                f = 1.8 * c + 32.0;
            }
            Temp::Fahrenheit(temp) => {
                f = *temp;
                c = (f - 32.0) / 1.8;
                k = c + 273.0;
                
            }
            Temp::Kelvin(temp) => {
                k = *temp;
                c = k - 273.0;
                f = 1.8 * c + 32.0;
            }
        };

        format!("{:.2}c {:.2}f {:.2}k", c, f, k)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("format invalid, please use % dg N[cfk]");
    }

    let mut input = args[1].clone();
    
    let mode = input.pop();
    let num: f64 = input.parse().expect("err parsing number");

    let temp = match mode  {
        Some('c') => Temp::Celsius(num),
        Some('f') => Temp::Fahrenheit(num),
        Some('k') => Temp::Kelvin(num),
        Some(_) | None => panic!("temp must end in c, f or k"),
    };

    println!("{}", temp.show_all());
}
