extern crate itertools;
use itertools::Flatten;
use std::io;
use std::cell::RefCell;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let x = 1;
    let o = 25;
    let gameDone = false;
    let linear_to_matrix = init_matrix();
    let mut _grid = RowGrid {
        square_grid : init_grid()
    };
    
    //while !gameDone {
    //_grid.occupy_square((0,0), 25);
    //_grid.occupy_square((0,1) , 25);
    _grid.occupy_square((0,2), 25);
    //}
    //if _grid.iterate_grid() {
    //    println!("Ok");
    //}
    //let _temp = _grid.clone();
    assert_eq!(true, _grid.check_there((0,0)).clone());
   //assert_eq!(true, _grid.clone().check_there((0,0)))
   // _grid.occupy_square((0,1),1); 
    
}


fn init_matrix() -> HashMap<u32, (u32,u32)> {
    type T = u32;
    let mut _hmap : HashMap<T, (T,T)> = HashMap::new(); 
    let mut index = 1;
    for i in 0..3 {
        for j in 0..3 {
            _hmap.insert(index, (i,j));
            index += 1;
        }  
    } 
    _hmap
}

fn init_grid() -> Vec<Vec<u32>> {
    let mut vec = Vec::new();
    for i in 0..3 {
        let mut temp_vec = Vec :: new();
        for j in 0..3 {
            temp_vec.push(0);
        }
        vec.push(temp_vec);
    }
    vec
}

fn linear_to_grid (_hmap : HashMap<u32, (u32, u32)>, _input : u32)  -> (u32,u32) {
     _hmap.get(&_input).unwrap().clone() //unwrap gets the value out of the option, clone "removes" the borrow (like deepcopy in Python)
} 

trait CheckGrid{
    fn square_grid(&self) -> Vec<Vec<u32>>;
}



#[derive(Debug, Clone)]
struct RowGrid  {
    square_grid : Vec<Vec<u32>>
}


impl RowGrid  {
    fn occupy_square(&mut self, _coordinates : (u32,u32), _player : u32) { 
        let foo : usize = _coordinates.0 as usize;
        let bar : usize = _coordinates.1 as usize;
        self.square_grid[foo][bar] += _player;
    }
    fn iterate_grid(self)-> bool { //example of generic data type
    
    for i in 0..3 {
        let temp_grid = self.square_grid[i].clone();
        if temp_grid.iter().fold(0, |foo,bar| foo + bar) == 3 {
            return true
        } else if temp_grid.iter().fold(0, |foo,bar| foo + bar) == 75 {
            return true
        } 
    }
    false
    }
    fn get_columns(self, index: usize) -> Vec<Vec<u32>> {
        let mut temp_grid = self.square_grid.clone();
        for i in 0..self.square_grid.len(){
            temp_grid[i] = self.square_grid[i].clone().
                into_iter().
                skip(index).
                step(3).
                collect();
        }
        temp_grid
    }
    fn get_diagonal(self) -> Vec<Vec<u32>>{
        let diagonal_one = vec!(self.square_grid[0][0], self.square_grid[1][1], self.square_grid[2][2]);
        let diagonal_two = vec!(self.square_grid[0][2], self.square_grid[1][1], self.square_grid[2][0]);
        let _empty = vec!(0,0,0);
        vec!(diagonal_one, diagonal_two, _empty)
    }
    fn print_grid(self) {
        let mut temp_grid = self.square_grid.clone();
        for i in 0..3 {
            for j in 0..3 {
                if temp_grid[i][j] != 0 {
                    if temp_grid[i][j] == 1 {
                        print!("X");
                    } else {
                        print!("O");
                    }
                } else {
                    print!("-");
                }
            }
            println!("");
        }
    }
    fn check_there(self, _coordinates : (u32,u32)) -> bool {
        let foo : usize = _coordinates.0 as usize;
        let bar : usize = _coordinates.1 as usize;
        if self.square_grid[foo][bar] == 0 {
            return true
        }
        false
    }
}








