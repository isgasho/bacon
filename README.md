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
#[macro_use]
extern crate bacon;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use bacon::Bacon;
use rand::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    gender: char
}

// encrypts a struct using the speck algorithm and decrypts it back
fn main() {
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(u128::min_value(), u128::max_value());

    let my = Person { name: "Alice".to_string(), age: 7, gender: 'f' };
    dbg!(&my);

    let fried_bacon: Bacon = fry!(my, key);
    dbg!(&fried_bacon);
    let p = unfry!(fried_bacon, Person, key);
    dbg!(p);
}
```