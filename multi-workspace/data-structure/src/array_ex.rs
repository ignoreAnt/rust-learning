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
