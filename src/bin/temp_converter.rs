fn main() {
    println!("Temperature Converter");

    let t1 = convert_temp("100rr", "f");

    match t1 {
        Ok(t) => println!("Tempreture Converted {:?}", t),
        Err(e) => println!("{:?}", e),
    }
}

fn convert_temp(tempreture: &str, unit: &str) -> Result<f32, String> {
    if unit.is_empty() {
        return Err(format!("Error: No input provided"));
    }

    let temp_f = match tempreture.parse::<f32>() {
        Ok(temp) => temp,
        Err(e) => {
            return Err(format!(
                "Error: Invalid temperature value: {}, {}",
                e, tempreture
            ));
        }
    };

    match unit.trim().to_ascii_lowercase().as_str() {
        "c" => Ok(temp_f * (9.0 / 5.0) + 32.0),
        "f" => Ok((temp_f - 32.0) * (5.0 / 9.0)),
        _ => {
            return Err(format!("Error: Invalid unit value: {}", unit));
        }
    }
}
