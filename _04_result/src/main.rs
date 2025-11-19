/**
 * @Author- Kavinda Rathnayake
 * @Date- 2025-11-19
 * @title- 04_result
 * @Description- This sub-directory is for practicing Rust Result.
 *               It contains a Cargo project with the necessary files.
 */

 // Example code for practicing Rust Result
 fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

// function that simulates fetch request that return Option
fn fetch_data(id: u32) -> Result<String, String> {
    let database = vec![
        (1, "Data for ID 1".to_string()),
        (2, "Data for ID 2".to_string()),
        (3, "Data for ID 3".to_string()),
    ];
    // Simulate fetching data
    for (db_id, data) in database {
        if db_id == id {
            return Ok(data);
        }
    }
    Err(String::from("Data not found"))
}

fn main() {
    
    println!("Division result: \n");
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    println!("Fetching data: \n");
    let data_id = 20;
    let data = fetch_data(data_id);
    match data {
        Ok(content) => println!("Fetched data: {}", content),
        Err(e) => println!("Error fetching data for ID {}: {}", data_id, e),
    }
}
