//! Home of the loading bar crate.
pub use colored::Color;
use std::{collections::HashMap, fmt};
pub mod loading_bar;
pub mod simple_loading_bar;
pub mod text_loading_bar;

#[derive(PartialEq, Clone, Copy)]
/// The type of change that is being made to the loading element (percentage or number)
pub enum Types {
    Percent,
    Index,
}

fn get_indexes(num: u16, left: u16, len: u16) -> HashMap<u16, u16> {
    let mut indexes = HashMap::new();
    // get the indexes based on left
    let done = len - left;
    let index = left / num;
    for i in 1..num {
        indexes.insert(done + (index * i), i);
    }
    indexes
}

fn get_index_and_value<T>(num: u16, left: u16, len: u16, value: &[T]) -> HashMap<u16, T>
where
    T: Clone,
{
    let mut index_and_value = HashMap::new();
    let done = len - left;
    let index = left / num;
    for i in 0..num {
        index_and_value.insert(done + (index * i), value[i as usize].clone());
    }
    index_and_value
}

fn generic_to_u16<T, U>(len: u16, change: HashMap<T, U>, type_of: Types) -> HashMap<u16, U>
where
    T: Copy + fmt::Debug,
    U: fmt::Debug + Clone,
    u16: From<T>,
    f32: From<T>,
{
    let mut change_color = HashMap::new();
    if Types::Percent == type_of {
        for (key, value) in change.iter() {
            let key: u16 = (len as f32 * f32::from(*key) / 100.0) as u16;
            change_color.insert(key, value.clone());
        }
    } else {
        for (key, value) in change.iter() {
            let change_key: u16 = u16::from(*key);
            change_color.insert(change_key, value.clone());
        }
    }
    change_color
}
