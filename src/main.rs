// Struct representing a sign
struct Sign {

    // Name of sign
    name: String, 

    // Array of things that it beats
    beats: Vec<String>,

}

// Implement sign struct
impl Sign {

    // Function to compare to another Sign (fight) - return true if self wins (false means undefined or lose)
    fn compare(&self, other: &Sign) -> bool {

        // Output var
        let mut result: bool = false;

        // Iterate through array
        for sign in self.beats.iter() {

            // If string matches
            if sign == &other.name {

                // Set result to true
                *&mut result = true;

            }

        }

        // Return result
        result

    }

    // Validate relationship with another sign (see if one beats the other - true is good)
    fn validate(&self, other: &Sign) -> bool {

        // If neither beats the other or they both beat each other
        if self.compare(other) == other.compare(self) {

            // Return false
            return false;

        }

        // Otherwise, return true
        true

    }

}

// Main function
fn main() {

    // TEST CODE
    let rock = Sign {

        name: String::from("rock"),
        beats: vec![String::from("scissors")],

    };

    let paper = Sign {

        name: String::from("paper"),
        beats: vec![String::from("rock")],

    };

    let scissors = Sign {

        name: String::from("scissors"),
        beats: vec![String::from("paper")],

    };

    println!("{}",rock.validate(&paper));

}

// test change