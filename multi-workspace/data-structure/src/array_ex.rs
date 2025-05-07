pub fn array_scores(index: usize, value: i32) {
    // Problem: Store the top 5 high scores for a game. Write functions to:
    // Initialize the scores (e.g., all to 0).
    // Update a score at a specific position (e.g., update the score at index 2).
    // Print all the scores.

    let mut scores: [i32; 5] = [0; 5];
    if let Some(score) = scores.get(index) {
        if *score != value {
            scores[index] = value
        }
    }

    println!("Scores: {:?}", scores);
}

pub fn while_array() {
    let mut scores: [i32; 5] = [0; 5];

    let mut current_index = 0;
    while let Some(score) = scores.get(current_index) {
        scores[current_index] = score + 100;
        current_index += 1
    }

    println!("Scores: {:?}", scores);
}

pub fn days_of_week(day_index: usize) {
    //     Problem: Create an array holding the names of the 7 days of the week. Write a function that takes a number (0-6) and returns the corresponding day name. Handle invalid input numbers gracefully (e.g., return None).
    // Why Array? There are always 7 days in a week. [&str; 7] is perfect.
    // Concepts: Array declaration (with &str), indexing [], Option enum, if/else or get() method for bounds checking.

    let week_days: [Days; 7] = [
        Days::Monday,
        Days::Tuesday,
        Days::Wednesday,
        Days::Thursday,
        Days::Friday,
        Days::Saturday,
        Days::Sunday,
    ];

    if let Some(day) = week_days.get(day_index - 1) {
        println!("Day: {:?}", day);
    } else {
        println!("Invalid day index: {}", day_index);
    }
}

#[derive(Debug)]
enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

pub fn calc_average() {
    // Calculate Average of Fixed Sensor Readings
    // Problem: You receive exactly 10 temperature readings from a sensor device. Store these in an array [f32; 10]. Calculate and return the average temperature.
    // Why Array? The number of readings (10) is fixed.
    // Concepts: Array declaration, iteration (for loop or iter()), summing elements, division, floating-point types.

    let mut sensor_readings: [f32; 10] = [0.0; 10];
    sensor_readings.fill(10.10);

    println!("Sensor Readings: {:?}", sensor_readings);

    let mut index = 0;
    let mut sum: f32 = 0.0;
    while let Some(temp) = sensor_readings.get(index) {
        sum += temp;
        index += 1
    }

    let avg = sum / sensor_readings.len() as f32;
    println!("Avg Temp: {:.2}", avg);

    let sum2: f32 = sensor_readings.iter().sum();
    let avg2 = sum2 / sensor_readings.len() as f32;
    println!("Avg2 Temp: {:.2}", avg2);
}

pub struct RGB {
    rgb: [u8; 3],
}

impl RGB {
    pub fn print_color(&self) {
        println!("R: {}, G: {}, B: {}", self.rgb[0], self.rgb[1], self.rgb[2]);
    }
}

pub fn new_color(r: u8, g: u8, b: u8) -> RGB {
    // Problem: Define a struct Color that holds an RGB color value using a fixed-size array [u8; 3] for the red, green, and blue components. Implement a simple function to create a Color.
    // Why Array? RGB has 3 fixed components. Embedding the array in a struct organizes the data.
    // Concepts: Struct definition, array field within a struct, function to instantiate the struct.
    RGB { rgb: [r, g, b] }
}
