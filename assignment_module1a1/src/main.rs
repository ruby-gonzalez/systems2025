
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}


fn main() {

    // F -> C
    let mut temp_f = 77.0;

    println!("{:.1}°F is {:.1}°C", temp_f, fahrenheit_to_celsius(temp_f));

    let mut count = 0;
    while count < 5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{:.1}°F is {:.1}°C", temp_f, temp_c);
        count +=1;
    }
    

    // C -> F
    let mut temp_c = 23.5;

    println!("{:.1}°C is {:.1}°F", temp_c, celsius_to_fahrenheit(temp_c));

    let mut count = 0;
    while count < 5 {
        temp_c += 1.0;
        let temp_f = celsius_to_fahrenheit(temp_c);
        println!("{:.1}°C is {:.1}°F", temp_c, temp_f);
        count +=1;
    }
}
