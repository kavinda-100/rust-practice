/**
 * @Author- Kavinda Rathnayake
 * @Date- 2025-11-19
 * @title- 02_enums
 * @Description- This sub-directory is for practicing Rust enums.
 *               It contains a Cargo project with the necessary files.
 */

// Define an Enum
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    // Rgb(u8, u8, u8),
}

// Implementation block for Color enum
impl Color {
    // Method to print the name of the color
    fn print_color(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
            // Color::Rgb(r, g, b) => println!("RGB Color: ({}, {}, {})", r, g, b),
        }
    }

    // get the color name as a string
    fn get_color_name(&self) -> &str {
        match self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            // Color::Rgb(_, _, _) => "RGB Color",
        }
    }
}

// Define a struct that uses the Color enum
#[derive(Debug)]
struct Vehicle {
    color: Color,
    model: String,
    year: u16,
}

// Implementation block for Vehicle struct
impl Vehicle {
    // Method to create a new Vehicle instance
    fn new(color: Color, model: String, year: u16) -> Self {
        Vehicle { color, model, year }
    }

    // Method to display vehicle information
    fn display_info(&self) {
        println!("Vehicle Model: {}, \nYear: {}", self.model, self.year);
        print!("Color: ");
        self.color.print_color();
    }
}

fn main() {
    
    // Create instances of Color enum
    let red_color = Color::Red;
    let green_color = Color::Green;

    // Print color names
    println!("printing colors:");
    red_color.print_color();
    green_color.print_color();

    println!();

    // Get color names as strings
    println!("Color names as strings:");
    println!("Red color name: {}", red_color.get_color_name());
    println!("Green color name: {}", green_color.get_color_name());

    println!();

    // Create instances of Vehicle struct
    let vehicle1 = Vehicle::new(Color::Red, String::from("Toyota"), 2020);
    let vehicle2 = Vehicle::new(Color::Green, String::from("Honda"), 2021);

    // Display vehicle information
    vehicle1.display_info();
    println!();
    vehicle2.display_info();
}
