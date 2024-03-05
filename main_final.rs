use std::collections::HashMap;
use std::io::{self, Write};

struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, description: String, price: f64, quantity: u32) {
        let product = Product {
            name: name.clone(),
            description,
            price,
            quantity,
        };
        self.products.insert(name, product);
        println!("Product added to inventory.");
    }

    fn edit_product(&mut self, name: &str, new_price: f64, new_quantity: u32) {
        if let Some(product) = self.products.get_mut(name) {
            product.price = new_price;
            product.quantity = new_quantity;
            println!("Product {} updated.", name);
        } else {
            println!("Product {} not found.", name);
        }
    }

    fn delete_product(&mut self, name: &str) {
        if let Some(_) = self.products.remove(name) {
            println!("Product {} deleted.", name);
        } else {
            println!("Product {} not found.", name);
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        println!("-----------------");
        for product in self.products.values() {
            println!("Name: {}", product.name);
            println!("Description: {}", product.description);
            println!("Price: ${}", product.price);
            println!("Quantity: {}", product.quantity);
            println!("-----------------");
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("Inventory Management System");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let name = get_user_input("Enter product name: ");
                let description = get_user_input("Enter product description: ");
                let price = get_user_input("Enter product price: ").parse::<f64>().unwrap_or_default();
                let quantity = get_user_input("Enter product quantity: ").parse::<u32>().unwrap_or_default();
                inventory.add_product(name, description, price, quantity);
            }
            2 => {
                let name = get_user_input("Enter product name to edit: ");
                let price = get_user_input("Enter new product price: ").parse::<f64>().unwrap_or_default();
                let quantity = get_user_input("Enter new product quantity: ").parse::<u32>().unwrap_or_default();
                inventory.edit_product(&name, price, quantity);
            }
            3 => {
                let name = get_user_input("Enter product name to delete: ");
                inventory.delete_product(&name);
            }
            4 => {
                inventory.generate_report();
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
