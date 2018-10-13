use std::collections::HashMap;
use std::io::{stdin};

fn main() {
    let mut game_grid = HashMap::new();
    let mut game_over = false;
    while !game_over {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read!!!");
        let input = input.trim();
        match input.parse::<u32>() {
            Ok(x) => game_grid = add_to_grid(game_grid, x, "X".to_string()),
            Err(y) => println!("Error Found : {}", y),
        };
    }
}

fn add_to_grid (mut _hashmap : HashMap<u32, String>, _coordinate : u32, _input : String) -> HashMap<u32, String> {
   _hashmap.entry(_coordinate).or_insert(_input);
   _hashmap
}

fn check_for_win (mut _hashmap : HashMap<u32, String>, mut _player : String) -> bool {
    for i in 1..3 {
        let nums :Vec<u32> = vec![1,2,3];
        let rows :Vec<u32> = nums.clone().into_iter().map(|x| x + i).collect();
        let cols :Vec<u32> = nums.clone().into_iter().map(|x| x * i).collect();
        if contains_all(rows, _hashmap.clone(), _player.clone()) || contains_all(cols, _hashmap.clone(), _player.clone()) {
            return true;
        }
    }
    false
}

fn contains_all (mut _nums : Vec<u32>, mut _hashmap : HashMap<u32,String>, mut _player : String) -> bool {
    for (_key, _val) in &_hashmap {
        if !(_hashmap.get(_key) == Some(&_player)) && _nums.contains(&_key) {
            return false
        }
    }
    true
}





