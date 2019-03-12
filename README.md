![Stage: dev](https://img.shields.io/badge/stage-dev-critical.svg "Stage:Development")
![rustc 1.33.0 (2aa4c46cf 2019-02-28)](https://img.shields.io/badge/rustc-stable%201.33.0%202aa4c46cf%202019--02--28-success.svg "rustc 1.33.0 (2aa4c46cf 2019-02-28)")

# bacon

Bacon makes it easy to en- and decrypt an arbitrary struct.

The library is currently under development, and currently only supports the Speck Cipher. 

## Usage

Bacon allows you to fry (encrypt) any struct of your code.

The provided ```struct Fryable``` stores a String message in ```data: Vec<String> ``` (ths may change to allow generic data types).

### Fry Bacon from arbitrary concrete struct

1. ```impl serde::Serialize``` for your struct

**Note**: This may change in the future (ie. #[derive(Bacon)])

*Example*: 

```rust
#[derive(Serialize)] // a derive should work for most "simple" structs
struct Person {
    name: String,
    age: u8,
    gender: Gender,  // ommitted has to derive serde::Serialize too
    address: String
}
```

2. Create a secret key of type ```u128```

**Note**: Bacon provides a utility function to create a u128 from ```&str```

*Examples*:

```rust
    let key_128: u128 = 1284736803748503;   // maybe not the best solution
    // or
    let key_128 = bacon::key_128(&args[1]); // a string secret from command line or any other source
```

3. Choose a frying method from two available:

  3.1. Using the provided macro ```fry!``` 
  3.2. Using the function ```Bacon::fry```

*Examples*:

```rust
    fry!(vip, key_128);       // 3.1 where vip is of T:Serialize
    // or
    Bacon::fry(vip, key_128); // 3.2. where vip is of T:Serialize
```

### Unfry Bacon

Unfrying a previously fried bacon works similarly easy.

1. Make sure the target type implements ```serde::Deserialize```

```rust
#[derive(Deserialize)] // a derive should work for most "simple" structs
struct Person { .. }
```

2. Unfry your bacon using one of two ways:

  2.1. Using macro: ```unfry!```
  2.2. Using ```Bacon::unfry::<T: Cipher>()``

**Note (only if you choose**: ```Bacon::unfry<T: Cipher, U:Deserialize>(fried_bacon, key_128)```.

There is currently no second Cipher option available, yet you have to pass ```bacon::Speck``` which implements  ```bacon::Cipher```.

*Examples*:

```rust
    match unfry!(bacon, Person, key_128) { // bincode::Result<T>
        Ok(p) => { dbg!(p); },
        Err(e) => { dbg!(e); }
    };
    // or
    let p: Person = Bacon::unfry<Speck, Person>(bacon, key_128).unwrap() // bincode::Result<T>
```

### Fry Bacon from a generic (String) message

**Note**: This may change in the future to generic payloads in ```Fryable```. Fryable is intended to store a payload of in a Vec<T>, that may include a Cipher declaration as well, to be send from to servers.

The provided ```struct bacon::Fryable``` which implements ```From<Vec<String>>``` can be used to fry a number of String messages without having to declare a struct on your own. It is therefore suitable to be used for messages from the command line.

1. Have a ```Vec<String>``` available
2. Create a Fryable from Vec<String>
3. Fry the fryable

*Example*:

```rust
    // $ cargo run --example command_line kfdkelf:elfkj4ef "Cipher/Speck" "This is a secret message"
    fn main() {
        // key from cli args
        let  mut args: Vec<String> = std::env::args().collect();
        let key_128 = bacon::key_128(&args[1]);
        args.drain(0..2);  // that is the program name and secret
        let fryable = Fryable::from(args);  
        let bacon = Bacon::fry(fryable, key_128);
        let f = Bacon::unfry::<Speck, Fryable>(bacon, key_128).unwrap();
        dbg!(f);
    }
```

## What else?

The fryable can be used to partially encrypt a struct of your choice. 

```rust
struct Person {
    id: u8,
    name: String.
    bank_account: Bacon  // encrypted bank account information
}

let fryable_bank_account = Fryable::from(
    vec![
        "First Moon Bank".to_string(),
        "IPBAN: M01A123456789".to_string()
    ]);
// fry bank account and add it to p
let fried_bank_account = Bacon::fry(fryable_bank_account, key_128);
let p = Person {
    id: 1234,
    name: "Dr Blofeld".to_string(),
    bank_account: fried_bank_account
};
dbg!(p);
```