use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pulse {
    Short,
    Long,
    None(char),
}

type Letter = Vec<Pulse>;

type Message = Vec<Letter>;

pub fn to_morse_code(s: &str) -> Message {
    use Pulse::*;
    let codes = HashMap::from([
        ('a', vec![Short, Long]),
        ('b', vec![Long, Short, Short, Short]),
        ('b', vec![Long, Short, Short, Short]),
        ('c', vec![Long, Short, Long, Short]),
        ('d', vec![Long, Short, Short]),
        ('e', vec![Short]),
        ('f', vec![Short, Short, Long, Short]),
        ('g', vec![Long, Long, Short]),
        ('h', vec![Short, Short, Short, Short, Short]),
        ('i', vec![Short, Short]),
        ('j', vec![Short, Long, Long, Long]),
        ('k', vec![Long, Short, Long]),
        ('l', vec![Short, Long, Short, Short]),
        ('m', vec![Long, Long]),
        ('n', vec![Long, Short]),
        ('o', vec![Long, Long, Long]),
        ('p', vec![Short, Long, Long, Short]),
        ('q', vec![Long, Long, Short, Long]),
        ('r', vec![Short, Long, Short]),
        ('s', vec![Short, Short, Short]),
        ('t', vec![Long]),
        ('u', vec![Short, Short, Long]),
        ('v', vec![Short, Short, Short, Long]),
        ('w', vec![Short, Long, Long]),
        ('x', vec![Long, Short, Short, Long]),
        ('y', vec![Long, Short, Long, Long]),
        ('z', vec![Long, Long, Short, Short]),
        ('1', vec![Short, Long, Long, Long, Long]),
        ('2', vec![Short, Short, Long, Long, Long]),
        ('3', vec![Short, Short, Short, Long, Long]),
        ('4', vec![Short, Short, Short, Short, Long]),
        ('5', vec![Short, Short, Short, Short, Short]),
        ('6', vec![Long, Short, Short, Short, Short]),
        ('7', vec![Long, Long, Short, Short, Short]),
        ('8', vec![Long, Long, Long, Short, Short]),
        ('9', vec![Long, Long, Long, Long, Short]),
        ('0', vec![Long, Long, Long, Long, Long]),
    ]);

    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| codes.get(&c).unwrap().clone())
        .collect()
}
