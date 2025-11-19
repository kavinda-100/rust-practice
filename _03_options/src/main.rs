/**
 * @Author- Kavinda Rathnayake
 * @Date- 2025-11-19
 * @title- 03_options
 * @Description- This sub-directory is for practicing Rust options.
 *               It contains a Cargo project with the necessary files.
 */

 // Example code for practicing Rust options
 fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// function that simulates fetch request that return Option
fn fetch_data(id: u32) -> Option<String> {
    let database = vec![
        (1, "Data for ID 1".to_string()),
        (2, "Data for ID 2".to_string()),
        (3, "Data for ID 3".to_string()),
    ];
    // Simulate fetching data
    for (db_id, data) in database {
        if db_id == id {
            return Some(data);
        }
    }
    None
}

 // Main function

fn main() {
    
    println!("Division result: \n");
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Division by zero"),
    }

    println!();

    println!("Fetching data: \n");
    let data_id = 20;
    let data = fetch_data(data_id);
    match data {
        Some(content) => println!("Fetched data: {}", content),
        None => println!("No data found for ID {}", data_id),
    }
}
