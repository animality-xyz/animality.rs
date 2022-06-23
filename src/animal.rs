use crate::InvalidAnimalError;
use std::{str::FromStr, fmt::Display};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Animal {
  Cat,
  Dog,
  Bird,
  Panda,
  Redpanda,
  Koala,
  Fox,
  Whale,
  Dolphin,
  Kangaroo,
  Bunny,
  Lion,
  Bear,
  Frog,
  Duck,
  Penguin,
  Axolotl,
  Capybara,
}

impl Animal {
  pub(crate) const fn as_str(&self) -> &'static str {
    match self {
      Self::Cat => "cat",
      Self::Dog => "dog",
      Self::Bird => "bird",
      Self::Panda => "panda",
      Self::Redpanda => "redpanda",
      Self::Koala => "koala",
      Self::Fox => "fox",
      Self::Whale => "whale",
      Self::Dolphin => "dolphin",
      Self::Kangaroo => "kangaroo",
      Self::Bunny => "bunny",
      Self::Lion => "lion",
      Self::Bear => "bear",
      Self::Frog => "frog",
      Self::Duck => "duck",
      Self::Penguin => "penguin",
      Self::Axolotl => "axolotl",
      Self::Capybara => "capybara"
    }
  }

  const fn from_str_impl(s: &str) -> Result<Self, InvalidAnimalError> {
    match s {
      "cat" => Ok(Animals::Cat),
      "dog" => Ok(Animals::Dog),
      "bird" => Ok(Animals::Bird),
      "panda" => Ok(Animals::Panda),
      "redpanda" => Ok(Animals::Redpanda),
      "koala" => Ok(Animals::Koala),
      "fox" => Ok(Animals::Fox),
      "whale" => Ok(Animals::Whale),
      "dolphin" => Ok(Animals::Dolphin),
      "kangaroo" => Ok(Animals::Kangaroo),
      "bunny" => Ok(Animals::Bunny),
      "lion" => Ok(Animals::Lion),
      "bear" => Ok(Animals::Bear),
      "frog" => Ok(Animals::Frog),
      "duck" => Ok(Animals::Duck),
      "penguin" => Ok(Animals::Penguin),
      "axolotl" => Ok(Animals::Axolotl),
      "capybara" => Ok(Animals::Capybara),
      _ => Err(InvalidAnimalError),
    }
  }
}

impl FromStr for Animal {
  type Err = InvalidAnimalError;

  #[inline(always)]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::from_str_impl(&s.to_lowercase())
  }
}

impl Display for Animal {
  #[inline(always)]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Animal({})", self.as_str())
  }
}

impl ToString for Animal {
  #[inline(always)]
  fn to_string(&self) -> String {
    self.as_str().to_string()
  }
}