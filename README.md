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

#[derive(Debug, Deserialize, Serialize)]
struct SuperSecretDocument<'a> {
    content: &'a str
}

impl <'a>SuperSecretDocument<'a> {
    fn new(content: &str) -> SuperSecretDocument {
        SuperSecretDocument { content }
    }
}

// encrypts a struct using the speck algorithm and decrypts it back
fn main() {
    // an arbitrary struct that implements Serialize and Deserialize
    let ssd = SuperSecretDocument::new("Hello World. How are you?");
    println!("{:#?}", &ssd);
    // Create a key, here a random u128
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(u128::min_value(), u128::max_value());
    // fry some bacon
    let fried_document: Bacon = fry!(ssd, key);
    println!("encrypted chunks: {:#?}", fried_document);
    
    // unfry the bacon
    unfry!(encr_chunks, SuperSecretDocument, key);
}
```