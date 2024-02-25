use std::collections::HashMap;

fn main(){
    let mut person_to_age:HashMap<&str,i32> = HashMap::new();

    person_to_age.insert("Rishit",22);
    person_to_age.insert("Thanuja",23);
    person_to_age.insert("Harshitha",21);

    println!("age of Rishit is: {}",person_to_age.get("Rishit").unwrap_or(&0));
    println!("age of Thanuja is: {}",person_to_age.get("Thanuja").unwrap_or(&0));
    println!("age of Harshitha is: {}",person_to_age.get("Harshitha").unwrap_or(&0));
    println!("age unknown person is: {}",person_to_age.get("Unknown").unwrap_or(&0));


    person_to_age.remove("Thanuja");
    println!("age of Thanuja is: {}",person_to_age.get("Thanuja").unwrap_or(&0));

    let is_existing = person_to_age.contains_key("Thanuja");
    println!("Is Thanuja on Dictionary: {}",is_existing);

    for (name,age) in &person_to_age{
        println!("name: {name} ; age : {age}");
    }

    // to overwrite value at a key we will just insert at the same key
    // this will overwrite value of key "Thanuja" is the key existed or not
    person_to_age.insert("Thanuja",21);
    person_to_age.insert("Thanuja",23);
    println!("age of Thanuja before safe update is: {}",person_to_age.get("Thanuja").unwrap_or(&0));

    // to insert value if the key exists else do not update the existing key value
    person_to_age.entry("Thanuja").or_insert(25);
    println!("age of Thanuja after safe update is: {}",person_to_age.get("Thanuja").unwrap_or(&0));
}