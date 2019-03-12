#[forbid(unsafe_code)]
extern crate bacon;
use bacon::{ Bacon, Fryable, Unfry, ciphers::Speck };

// $ cargo run --example challenge {16-digit-key}
// Example: cargo run --example challenge u.ijd.3HH8$n.MhK
fn main() {
    let key_str: Vec<String> = std::env::args().collect();
    let key_u128 = bacon::key_128(&key_str[1]);
    let bacon =  Bacon {
        data: vec![
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
    };
    let fryable = Bacon::unfry::<Speck, Fryable>(bacon, key_u128);
    match fryable {
        Ok(f) => { dbg!(f); },
        Err(e) => { dbg!(e); },
    }
}
