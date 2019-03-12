# Bacon Challenge

Speck has been developed by the NSA (National Security Agency) of the USA and has since publishing caused lots of noise, raised doubts. 

It has been called deeply flawed and backdoors may have been implemented.

I am not a Cryptologist and cannot evaluate the claims. Therefore I present this challenge. 

Show me that the Speck algorhithm or the implementation used by bacon can be cracked. 

1. This is the result of a partially fried ```struct Challenge```:

```rust
Challenge {
    id: 1,
    description: "Try to decrypt the fried message and follow the instructions if successfully hacked.",
    secret_message: Bacon {
        data: [
            121388295326026093385741144774451129613,
            235733079392620015913199396004081179027,
            91687230068729700441490464072911997800,
            149700271366331893028270861235626549396,
            175569350845875193932221787957796681385,
            4000777898746445739938620310012244771,
            304804736351933226035277407036977781670,
            150433573937954520190284610719857599262,
            64963353127757738897286809233862151788,
            121870591213294879058267532996899949678,
            30181629679726203095081156651117825360,
            304603002802075187003108587254315915953,
            339747025455216280778164507700038605350
        ]
    }
}
```

2. This is the ```struct Challenge```:

```rust
struct Challenge {
    id: u8,
    description: String,
    secret_message: Bacon,
}
```

3. The way I fried the secret message of the Challenge.

The secret key is obviously *"lost"*. The original key was a 16 digit string of a-z A-Z 0-9 and special characters you find on any keyboard. No language specific characters are used. (ie. no ä è ß etc.), but could contain characters of Shift+1-0 or ,.-;:_+#*'. No return, newline, break characters are used. 

```rust
// create fryable message
let fryable_msg = Fryable::from(
    vec![
        "...".to_string(),
        // .. several other Strings 
    ]);

// frying message
let bacon = Bacon::fry(fryable_msg, key_u128); // 

// creating Challenge object
let challenge = Challenge {
    id: 1,
    description: "Try to decrypt the fried message and follow the instructions if successfully hacked.".to_string(), 
    secret_message: bacon
};
```
4. If your approach is to find (or bruteforce) the secret key you should be able to unfry the secret message with

```rust
let secret_fryable: Fryable = Bacon::unfry::<Speck, Fryable>(challenge.secret_message, {THE_BRUTE_FORCED_KEY as u128} ).unwrap();
```
    
5. I have provided an example code snippet which allows you to decrypt the message, if you have found the correct key.

You can run the example from the command line:

```rust
// $ cargo run --example challenge {16-digit-key}
// Example: cargo run --example challenge u.ijd.3HH8$n.MhK
```

If the provided key is wrong you will receive an error message:

```rust
[examples/challenge.rs:28] e = Io(
    Custom {
        kind: UnexpectedEof,
        error: StringError(
            ""
        )
    }
)

```