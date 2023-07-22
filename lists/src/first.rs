// src\first.rs

pub enum List {
    Empty,
    Elem(i32, Box<List>),
}