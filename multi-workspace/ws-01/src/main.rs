use anyhow::{Context, Ok, Result};
fn main() {
    let p1 = Person::new("Rupesh", 70.0, 1.75);
    let bmi = p1.get_bmi().context("Failed to calculate BMI");
    println!("Bmi for {:?} = {:?}", p1.name, bmi.unwrap_or_default());

    let p2 = Person::new("xxx", 0.0, -1.2);
    let bmi2 = p2.get_bmi().context("Failed to calculate BMI");
    println!("Bmi for {:?} = {:?}", p2.name, bmi2.unwrap_or_default());
}

struct Person {
    name: String,
    weight: Option<f64>,
    height: Option<f64>,
}

impl Person {
    fn new(name: &str, weight: f64, height: f64) -> Self {
        Person {
            name: name.to_string(),
            weight: Some(weight),
            height: Some(height),
        }
    }

    fn get_bmi(&self) -> Result<f64> {
        if self.weight.is_none() || self.height.is_none() {
            return Err(anyhow::anyhow!("Missing Weight or Hieght"));
        };

        let w = self.weight.unwrap_or_default();
        let h = self.height.unwrap_or_default();

        if w <= 0.0 || h <= 0.0 {
            return Err(anyhow::anyhow!("Invalid height or weight"));
        }

        let bmi = w / (h * h);
        Ok(bmi)
    }
}
