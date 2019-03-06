#[forbid(unsafe_code)]
#[macro_use]
extern crate bacon;
extern crate bincode;
extern crate rand;
extern crate persy;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use bacon::{ Bacon, Key };
use bincode::{serialize, deserialize};
use persy::{ Config, Persy, PersyId, PersyError, PRes };
use rand::{ distributions::{ Alphanumeric }, Rng };
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Gender { Female, Male, Null }
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
    address: String,
    description: String
}

impl Person {
    fn empty() -> Person {
        Person { name: String::new(), age: u8::min_value(), gender: Gender::Null, address: String::new(), description: String::new() }
    }
}

struct DB;

impl DB {
    fn open<P: Into<String> + Debug + Clone>(path: P) -> Result<Persy, PersyError> {
        match Persy::open(path.clone(), Config::new()) {
            Ok(p) => {
                println!("persy opened.");
                Ok(p)
            },
            Err(e) => { 
                dbg!(e);
                println!("Trying to create: {:?}", path);
                match Persy::create(path.clone()) {
                    Err(e) => { dbg!(&e); Err(e) },
                    Ok(_) => {
                        println!("persy default created.");
                        println!("Trying to open: {:?}", path);
                        let persy = Persy::open(path, Config::new())?;
                        println!("persy opened.");
                        let mut tx = persy.begin()?;
                        persy.create_segment(&mut tx, "default")?; 
                        let prepared = persy.prepare_commit(tx)?;
                        persy.commit(prepared)?;
                        Ok(persy)
                    }
                }
            }
        }
    }
    fn read(persy: &Persy, id: PersyId) -> Result<Vec<u8>, PersyError> {
        let mut tx = persy.begin()?;
        let read = persy.read_record_tx(&mut tx, "default", &id)?.expect("record exists");
        Ok(read)
    }
    fn write(persy: &Persy, data: &[u8]) -> Result<PersyId, PersyError> {
        let mut tx = persy.begin()?;
        let id = persy.insert_record(&mut tx, "default", data)?;
        let prepared = persy.prepare_commit(tx)?;
        persy.commit(prepared);
        dbg!(&id);
        Ok(id)
    }
}

// encrypts a struct using the speck algorithm and decrypts it back
// $ cargo run --example persy { persy_storage_file } { optional 16 character pass } 
fn main() {
    // key
    let args: Vec<String> = std::env::args().collect();

    let mut key_str: String = String::new();
    let mut path: String = String::new();
    path = args[1].clone();
    dbg!(&args[1]);
    if args.len() > 2 {
        key_str = args[2].clone();
        drop(args);
    } else {
        let mut rng = rand::thread_rng();
        key_str = rng.sample_iter(&Alphanumeric).take(16).collect();
    }
    let key_128 =  bacon::key_128(&key_str);
    key_str = "".to_string();
    drop(key_str);
    
    // create struct
    println!("Creating a struct");
    let mut vip = Person {
        name: "Ernst Stavro Blofeld".to_string(),
        age: 77,
        gender: Gender::Male,
        address: "Inside a Vulcano, Japan".to_string(),
        description: "CEO of SPECKTRE aka Bacon Industries".to_string()
    };
    dbg!(&vip);
    // fry struct
    println!("Fry a struct");
    let fried_bacon: Bacon = fry!(vip, key_128);
    dbg!(&fried_bacon);
    vip = Person::empty();
    drop(vip);

    // persist bacon
    // open or create and open persy storage
    println!("Trying to open: {:?}", path);
    let persy = DB::open(path).unwrap();
    // persy fried bacon into persy.storage
    match DB::write(&persy, &serialize(&fried_bacon).unwrap()) {
        Ok(id) => {
            drop(fried_bacon);
            // read persyfied bacon
            println!("Ready persyfied bacon");
            let read = DB::read(&persy, id);
            println!("Persified bacon read.");
            dbg!(&read);
            // deserialize into encrypted Bacon
            println!("Deserializing bacon");
            let bacon: Bacon = deserialize(&read.unwrap()).unwrap();
            println!("Bacon deserialized");
            dbg!(&bacon);
            println!("Unfry(decrypt) bacon into Person");
            match unfry!(bacon, Person, key_128) {
                Ok(p) => { dbg!(p); },
                Err(e) => { dbg!(e); }
            }
        },
        Err(e) => { dbg!(e); }
    }
    

 
    // load fried bacon from persy.storage
     
    // decrypt attempt with correct key
   

    // println!();
}