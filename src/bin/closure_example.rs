use std::collections::HashMap;

fn main() {
    let mut inventory = Inventory::new();

    println!("{:#?}", inventory);

    let mut result = inventory.giveaway_one_shirt(Some(ShirtColor::Red));
    println!("{:#?}", result);
    println!("{:#?}", inventory);

    result = inventory.giveaway_one_shirt(None);
    println!("{:#?}", result);
    println!("{:#?}", inventory);

    result = inventory.giveaway_one_shirt(None);
    println!("{:#?}", result);
    println!("{:#?}", inventory);

    result = inventory.giveaway_one_shirt(None);
    println!("{:#?}", result);
    println!("{:#?}", inventory);
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum ShirtColor {
    Red,
    Green,
    Blue,
}

impl ShirtColor {
    fn values() -> Vec<ShirtColor> {
        vec![ShirtColor::Red, ShirtColor::Green, ShirtColor::Blue]
    }
}

#[derive(Debug)]
struct Inventory {
    shirts: HashMap<ShirtColor, i32>,
}

impl Inventory {
    fn new() -> Self {
        let mut shirts_inventory = HashMap::new();

        for shirtcolor in ShirtColor::values() {
            shirts_inventory.insert(shirtcolor, 1);
        }

        let inventory = Self {
            shirts: shirts_inventory,
        };

        inventory
    }

    fn giveaway_one_shirt(
        &mut self,
        user_preference: Option<ShirtColor>,
    ) -> Result<&ShirtColor, String> {
        match user_preference {
            Some(preference) => {
                let preference_availability = self.shirts.iter_mut().find(|shirt_invertory| {
                    *shirt_invertory.0 == preference && *shirt_invertory.1 > 0
                });

                match preference_availability {
                    Some(shirt_invertory) => {
                        *shirt_invertory.1 -= 1;
                        Ok(shirt_invertory.0)
                    }
                    None => Err(format!("Sorry {:?} is not available", preference)),
                }
            }
            None => {
                let highly_available_shirt_inventory = self
                    .shirts
                    .iter_mut()
                    .max_by_key(|shirt_invertory| *shirt_invertory.1)
                    .filter(|shirt_invertory| *shirt_invertory.1 > 0);

                match highly_available_shirt_inventory {
                    Some(shirt_inventory) => {
                        *shirt_inventory.1 -= 1;
                        Ok(shirt_inventory.0)
                    }
                    None => Err("No shirts available".to_string()),
                }
            }
        }
    }
}
