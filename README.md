# bacon

Bacon makes it easy to en- and decrypt an arbitrary struct.

The library is currently undert development, only supports the Speck Cipher is supported. 

## Usage

Bacon allows you to fry (encrypt) and struct of your code. Bacon provides a ```struct Fryable``` that can stores a message in ```data: Vec<String> ``` (ths may change to allow generic data types).

### Fry Bacon from arbitrary concrete struct

1. ```impl serde::Serialize``` for your struct

**Note**: This may change in the future (ie. #[derive(Bacon)])

*Example*: 

```rust
#[derive(Serialize)] // a derive should work for most "simple" structs
struct Person {
    name: String,
    age: u8,
    gender: Gender,
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

Unfrying a previously fried bacon works similarly.

**Note**: There is currently no second Cipher available, yet you have to pass "Speck" that implements the trait bacon::Cipher, if you use

```Bacon::unfry<T: Cipher, U:Deserialize>(fried_bacon, key_128)```

1. Make sure the target type implements ```serde::Deserialize```

```rust
#[derive(Deserialize)] // a derive should work for most "simple" structs
struct Person { .. }
```

2. Unfry your bacon using one of two ways:

2.1. Using macro: ```unfry!```
2.2. Using ```Bacon::unfry::<T: Cipher>()``

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

**Note**: This may change in the future to allow to store generic Types in ```Fryable``` 

THe provided ```struct bacon::Fryable```, which implements ```From<Vec<String>>```, can be used to fry String messages without having to declare a struct on your own. It is therefore suitable to be used for messages from the command line.

1. Have a ```Vec<String>``` available
2. Create a Fryable from Vec<String>
3. Fry the fryable

*Example*:

```rust
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