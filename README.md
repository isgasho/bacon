# bacon

Provides two macros ```fry!``` and ```unfry!``` to serialize and encrypt arbitrary structs.

## Usage

### Fry Bacon

Encrypt an arbitrary struct

1. impl ```Deserialize``` and ```Serialize``` (crate serde) for your struct
2. Create secret as u128
3. Invoke ```fry!(my_struct, key)``` to encrypt your item

### Unfry Bacon

Decrypt a Bacon object

1. Invoke ```unfry!(encrypted_item, key)```  passing your bacon object and the key used to encrypt your struct


### examples/main.rs

This is the provided exmample:

```rust
#[forbid(unsafe_code)]
#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bacon::Bacon;
use rand::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Gender { Female, Male }
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
    address: String
}

// encrypts a struct using the speck algorithm and decrypts it back
fn main() {
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(u128::min_value(), u128::max_value());
    
    println!("Creating a struct");

    let my = Person {
        name: "Sophia Pia Du Hamel".to_string(),
        age: 23,
        gender: Gender::Female,
        address: "7 Park Avenue, Olympus Mons, Mars".to_string()
    };
    dbg!(&my);
    println!();

    // fry struct
    let fried_bacon: Bacon = fry!(my, key);
    println!("Encrypted struct \"Fried Bacon\"");
    dbg!(&fried_bacon);
    println!();

    // decrypt attempt with wrong key
    println!("Attempt to decrypt with wrong key.");
    let wrong_key = rng.gen_range(u128::min_value(), u128::max_value());
    let fried_clone = fried_bacon.clone();
    match unfry!(fried_clone, Person, wrong_key) {
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    }
    println!();

    // decrypt attempt with correct key
    println!("Attempt to decrypt with correct key.");
    match unfry!(fried_bacon, Person, key) {
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    }
}
```