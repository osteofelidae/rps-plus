// Struct representing a sign
#[derive(Clone)]
struct Sign {

    // Name of sign
    name: String, 

    // Array of things that it beats
    beats: Vec<String>,

}

// Implement sign struct
impl Sign {

    // TODO Function to create one

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

    // Validate with a vector of signs
    fn validate_multiple(&self, others: &Vec<Sign>) -> (bool, Vec<Sign>) {

        // Output variables
        let mut invalid_signs: Vec<Sign> = Vec::new();

        // Iterate through input vector
        for sign in others {

            // If sign does not validate
            if !self.validate(sign) {

                // Add to output vector
                invalid_signs.push(sign.clone());

            }

        }

        // Set result to whether values exist in invalid list
        let result: bool = !(invalid_signs.len() > 0);

        // Return result tuple
        (result, invalid_signs)

    }

}

// Main function
fn main() {

    // TEST CODE
    let rock = Sign {

        name: String::from("rock"),
        beats: vec![String::from("scissors"), String::from("paper")],

    };

    let paper = Sign {

        name: String::from("paper"),
        beats: vec![String::from("rock")],

    };

    let scissors = Sign {

        name: String::from("scissors"),
        beats: vec![String::from("paper"), String::from("rock")],

    };

    println!("{}",rock.validate(&paper));
    let test: Vec<Sign> = vec![paper, scissors];
    println!("{}, {}",rock.validate_multiple(&test).0, rock.validate_multiple(&test).1[1].name);

}