use std::env;

enum Temp {
    Celsius(f64), 
    Fahrenheit(f64), 
    Kelvin(f64)
}

impl Temp {
    fn get_cfk(&self) -> (f64, f64, f64) {
        match *self {
            Temp::Celsius(temp) => (temp, 1.8 * temp + 32.0, temp + 273.0),
            Temp::Fahrenheit(temp) => ((temp - 32.0) / 1.8, temp, (temp - 32.0) / 1.8 + 273.0),
            Temp::Kelvin(temp) => (temp - 273.0, 1.8 * (temp - 273.0) + 32.0, temp)
        }
    }
}

fn main() {
    let mut args = env::args();
    let mut input: String;

    if let None = args.next() { panic!("Some wrong is not right"); }
    if let Some(arg) = args.next() {
        input = arg.clone()
    } else {
        panic!("format input, please use % dg N[cfk]");
    }
    
    let mode = input.pop();
    let num: f64 = input.parse().expect("err parsing number");

    let temp = match mode  {
        Some('c') => Temp::Celsius(num),
        Some('f') => Temp::Fahrenheit(num),
        Some('k') => Temp::Kelvin(num),
        Some(_) | None => panic!("temp must end in c, f or k"),
    };

    let (c, f, k) = temp.get_cfk();

    println!("{:.2}c {:.2}f {:.2}k", c, f, k);
}
