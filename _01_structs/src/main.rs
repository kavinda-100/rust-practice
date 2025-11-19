/**
 * @Author- Kavinda Rathnayake
 * @Date- 2025-11-19
 * @title- 01_structs
 * @Description- This sub-directory is for practicing Rust structs.
 *               It contains a Cargo project with the necessary files.
 */

 // Define a struct named Person with various fields
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    is_active: bool,
}

// implementation block for Person struct
impl Person {
    // Method to create a new Person instance
    fn new(first_name: String, last_name: String, age: u8, email: String, is_active: bool) -> Self {
        Person {
            first_name,
            last_name,
            age,
            email,
            is_active,
        }
    }

    // Method to update the person's information
    fn update_info(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        age: Option<u8>,
        email: Option<String>,
        is_active: Option<bool>,
    ) -> &mut Self {
        match first_name {
            Some(fname) => self.first_name = fname,
            None => {}
        }
        match last_name {
            Some(lname) => self.last_name = lname,
            None => {}
        }
        match age {
            Some(a) => self.age = a,
            None => {}
        }
        match email {
            Some(e) => self.email = e,
            None => {}
        }
        match is_active {
            Some(active) => self.is_active = active,
            None => {}
        }
        self
    }

    // Method to display full person information
    fn display_info(&self) {
        println!(
            "Name: {} {}, \nAge: {}, \nEmail: {}, \nActive: {}",
            self.first_name, self.last_name, self.age, self.email, self.is_active
        );
    }
}

fn main() {
    // Create a new Person instance
    let mut person = Person::new(
        String::from("Kavinda"),
        String::from("Rathnayake"),
        24,
        String::from("kavinda@example.com"),
        true,
    );

    // Display the person's information
    person.display_info();

    // Update the person's information
    person.update_info(None, None, None, Some(String::from("newemail@example.com")), None);

    // Display the updated person's information
    println!("\nAfter update:");
    
    person.display_info();
}
