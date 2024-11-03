use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, from_str};

#[derive(Serialize, Deserialize, Debug)]
struct Dog {
    name: String,
    year_born: i32,
    owner: DogOwner
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner {
    first_name: String,
    last_name: String,
}

fn main() {
    deserialize();
    serialize_test();
}

fn serialize_test() {
    let owne01 = DogOwner{first_name: "Sujith".to_string(), last_name: "V I".to_string()};
    let dog01 = Dog{name: "Sasi".to_string(), year_born: 2011, owner: owne01};
    let dog_ser = to_string_pretty(&dog01);
    if dog_ser.is_ok() {
        println!("{}", dog_ser.ok().unwrap());
    }else {
        println!("{:#?}", dog_ser.unwrap_err());
    }
}


fn deserialize() {
    let raw_str = r#"
    {
        "name": "Sasi",
        "year_born": 2011,
        "owner": {
            "first_name": "Sujith",
            "last_name": "V I"
        }                      
    }
    "#;

    let json_dog = from_str::<Dog>(raw_str);

    if json_dog.is_ok() {
        println!("{:#?}",json_dog.ok().unwrap());
    }else{
        println!("{:#?}", json_dog.unwrap_err());
    }
}