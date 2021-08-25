mod rot13;

use std::io::{self, prelude::*};
use rot13::rot13;
use std::fs::OpenOptions;

fn main() {
    choice();
}

fn sign_up() {
    println!("Enter username:");

    let mut username = String::new();

    io::stdin().read_line(&mut username)
        .expect("Failed to read line.");

    println!("Enter password:");

    let mut password = String::new();

    io::stdin().read_line(&mut password)
        .expect("Failed to read line.");

    let password = rot13(&password);

    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open("credentials.txt") {
            Ok(content) => content,
            Err(_) => panic!("Could not create file.")
        };

    let credentials = format!("{}{}", username, password);

    match file.write_all(credentials.as_bytes()) {
        Ok(_) => println!("Successfully signed up."),
        Err(_) => panic!("Could not write to file.")
    }
}

fn login() {
    let mut file = match OpenOptions::new()
        .read(true)
        .open("credentials.txt") {
            Ok(content) => content,
            Err(_) => panic!("Could not create file.")
        };

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let split: Vec<&str> = content.split("\n").collect();

    let _username = get_username(&split);

    let _password = get_password(&split);

    println!("Successfully logged in!");
}

fn choice() {
    loop {
        println!("Would you like to (s)ign up or (l)ogin:");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim() {
            "s" => sign_up(),
            "l" => { login(); break },
            _ => continue
        };
    }
}

fn get_username(split: &Vec<&str>) -> String {
    loop {
        println!("Enter your username.");

        let mut username = String::new();

        io::stdin().read_line(&mut username)
            .expect("Failed to read line.");

        match split.iter().find(|&&x| x.trim() == username.trim()) {
            Some(_) => return username,
            None => { println!("Invalid username."); continue }
        };
    }
}

fn get_password(split: &Vec<&str>) -> String {
    loop {
        println!("Enter your password.");

        let mut password = String::new();

        io::stdin().read_line(&mut password)
            .expect("Failed to read line");
        
        match split.iter().find(|&&x| rot13(&x.to_string()).trim() == password.trim()) {
            Some(_) => return password,
            None => { println!("Invalid password."); continue }
        };
    }
}
