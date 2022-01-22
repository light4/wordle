use crate::error::InputError;
use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use rand::{self, prelude::SliceRandom};

pub const LENGTH: usize = 5;

fn global_dict() -> &'static Vec<&'static str> {
    static INSTANCE: OnceCell<Vec<&str>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        include_str!("../assets/words.txt")
            .split_whitespace()
            .collect::<Vec<&str>>()
    })
}

fn random_word() -> String {
    let mut rng = rand::thread_rng();
    global_dict()
        .choose(&mut rng)
        .expect("choose random word error")
        .to_string()
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone)]
pub struct World {
    pub result: Vec<char>,
    pub cursor: (usize, usize),
    pub grid: Vec<Vec<Item>>,
    pub characters: Vec<Item>,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterState {
    Untouch,
    Buffer,
    Right,
    WrongPos,
    Wrong,
}

impl Default for CharacterState {
    fn default() -> Self {
        Self::Untouch
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub inner: char,
    pub state: CharacterState,
}

impl Item {
    fn new(inner: char, state: CharacterState) -> Self {
        Self { inner, state }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            inner: ' ',
            state: CharacterState::Untouch,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(random_word(), "qwertyuiopasdfghjklzxcvbnm".to_owned())
    }
}

impl World {
    pub fn new(w: String, characters: String) -> Self {
        let mut grid = vec![];
        for _ in 0..LENGTH {
            grid.push([Item::default(); LENGTH].to_vec());
        }

        Self {
            result: w.chars().collect(),
            cursor: (0, 0),
            grid,
            characters: characters
                .chars()
                .map(|c| Item::new(c, CharacterState::Untouch))
                .collect(),
        }
    }

    pub fn reset(&mut self) {
        let new = World::default();
        self.result = new.result;
        self.cursor = (0, 0);
        self.grid = new.grid;
        self.characters = new.characters;
    }

    pub fn enter(&mut self) -> Result<()> {
        let s = self.check_input()?;

        for (idx, c) in s.chars().enumerate() {
            if self.result.contains(&c) {
                if self.result[idx] == c {
                    self.grid[self.cursor.0][idx].state = CharacterState::Right;
                    for i in self.characters.iter_mut() {
                        if i.inner == c {
                            i.state = CharacterState::Right;
                        }
                    }
                } else {
                    self.grid[self.cursor.0][idx].state = CharacterState::WrongPos;
                    for i in self.characters.iter_mut() {
                        if i.inner == c && i.state != CharacterState::Right {
                            i.state = CharacterState::WrongPos;
                        }
                    }
                }
            } else {
                self.grid[self.cursor.0][idx].state = CharacterState::Wrong;
                for i in self.characters.iter_mut() {
                    if i.inner == c {
                        i.state = CharacterState::Wrong;
                    }
                }
            }
        }
        self.cursor.0 += 1;
        self.cursor.1 = 0;

        Ok(())
    }

    pub fn input_char(&mut self, input: char) {
        let current = self.grid[self.cursor.0][self.cursor.1];
        // 当前指向的方格已经填了
        if (self.cursor.1 + 1) < LENGTH && current.state == CharacterState::Buffer {
            self.cursor.1 += 1;
        }
        self.grid[self.cursor.0][self.cursor.1] = Item::new(input, CharacterState::Buffer);
    }

    pub fn delete_char(&mut self) {
        self.grid[self.cursor.0][self.cursor.1] = Item::default();
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        }
    }

    fn check_input(&self) -> Result<String> {
        if (self.cursor.1 + 1) != LENGTH {
            bail!(InputError::NotEnoughLetters);
        }
        let s: String = self.grid[self.cursor.0].iter().map(|i| i.inner).collect();
        if !global_dict().contains(&s.as_ref()) {
            bail!(InputError::NotInWordList);
        }

        Ok(s)
    }
}
