use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn build_from_csv(filepath: &str) -> Vec<Box<Person>>{
        println!("{}", filepath);
    let mut dic: Vec<Box<Person>> = vec![];
    for line in read_to_string(filepath).unwrap().lines() {

        //println!("{}", line);
        let v: Vec<_> = line.split(',').collect();
        let data = Box::new(Person {
            name: v[0].to_string(),
            age: v[1].to_string().parse().unwrap(),
        });
        dic.push(data);
    }
    println!("{:?}", dic);

    return dic;
}

fn build_from_json(json_data: &str) {
    let pv: Vec<Person> = serde_json::from_str(json_data).unwrap();

    let mut dic: Vec<Box<Person>> = vec![];

    for data in &pv {
        //println!("{:?}", data);
        let data = Box::new(Person {
            name: data.name.clone(),
            age: data.age,
        });
        dic.push(data);
    }
    println!("{:?}", dic);
}

fn build_basic() {
    let mut dic: Vec<Box<Person>> = vec![];

    let data = Box::new(Person {
        name: String::from("gusa"),
        age: 30,
    });
    println!("{:?}", data);
    dic.push(data);

    let data = Box::new(Person {
        name: String::from("terada"),
        age: 37,
    });
    println!("{:?}", data);
    dic.push(data);

    println!("{:?}", dic);
}

fn main() {
    println!("buid_basic -------------------------------");
    build_basic();

    let json_data = "
      [
        {\"id\" : \"1\", \"name\" : \"gusa\",   \"age\" : 32},
        {\"id\" : \"2\", \"name\" : \"sudo\",   \"age\" : 22},
        {\"id\" : \"3\", \"name\" : \"peal\",   \"age\" : 52},
        {\"id\" : \"4\", \"name\" : \"yamamoto\",   \"age\" : 45},
        {\"id\" : \"5\", \"name\" : \"araki\",   \"age\" : 38},
        {\"id\" : \"6\", \"name\" : \"ogihara\",   \"age\" : 32},
        {\"id\" : \"7\", \"name\" : \"nakata\", \"age\" : 48}
      ]
      ";

    println!("buid_from_json -------------------------------");
    build_from_json(json_data);

    println!("buid_from_csv -------------------------------");
    let csv = "./test.csv";
    let vec = build_from_csv(csv);
    println!("{} {}", vec[0].name,   vec[0].age);
    println!("{} {}", vec[4].name,   vec[4].age);
}
