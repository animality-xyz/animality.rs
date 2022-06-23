use crate::InvalidAnimalError;
use std::{str::FromStr, fmt::Display};

/// Represents a list of animals supported by the Animality API.
/// 
/// List of supported animals:
/// - `cat` (`Animal::Cat`)
/// - `dog` (`Animal::Dog`)
/// - `bird` (`Animal::Bird`)
/// - `panda` (`Animal::Panda`)
/// - `redpanda` (`Animal::Redpanda`)
/// - `koala` (`Animal::Koala`)
/// - `fox` (`Animal::Fox`)
/// - `whale` (`Animal::Whale`)
/// - `dolphin` (`Animal::Dolphin`)
/// - `kangaroo` (`Animal::Kangaroo`)
/// - `bunny` (`Animal::Bunny`)
/// - `lion` (`Animal::Lion`)
/// - `bear` (`Animal::Bear`)
/// - `frog` (`Animal::Frog`)
/// - `duck` (`Animal::Duck`)
/// - `penguin` (`Animal::Penguin`)
/// - `axolotl` (`Animal::Axolotl`)
/// - `capybara` (`Animal::Capybara`)
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,norun
/// extern crate animality;
/// use animality::Animal;
/// 
/// let dog = Animal::Dog;
/// ```
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Animal {
  /// Represents the animal `cat`.
  Cat,
  /// Represents the animal `dog`.
  Dog,
  /// Represents the animal `bird`.
  Bird,
  /// Represents the animal `panda`.
  Panda,
  /// Represents the animal `redpanda`.
  Redpanda,
  /// Represents the animal `koala`.
  Koala,
  /// Represents the animal `fox`.
  Fox,
  /// Represents the animal `whale`.
  Whale,
  /// Represents the animal `dolphin`.
  Dolphin,
  /// Represents the animal `kangaroo`.
  Kangaroo,
  /// Represents the animal `bunny`.
  Bunny,
  /// Represents the animal `lion`.
  Lion,
  /// Represents the animal `bear`.
  Bear,
  /// Represents the animal `frog`.
  Frog,
  /// Represents the animal `duck`.
  Duck,
  /// Represents the animal `penguin`.
  Penguin,
  /// Represents the animal `axolotl`.
  Axolotl,
  /// Represents the animal `capybara`.
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
}

impl FromStr for Animal {
  type Err = InvalidAnimalError;

  /// Converts a string to an [`Animal`]. This conversion is case-insensitive.
  /// 
  /// # Errors
  /// 
  /// Returns an [`InvalidAnimalError`] if the lowercased string is neither of the following:
  /// - `cat`
  /// - `dog`
  /// - `bird`
  /// - `panda`
  /// - `redpanda`
  /// - `koala`
  /// - `fox`
  /// - `whale`
  /// - `dolphin`
  /// - `kangaroo`
  /// - `bunny`
  /// - `lion`
  /// - `bear`
  /// - `frog`
  /// - `duck`
  /// - `penguin`
  /// - `axolotl`
  /// - `capybara`
  /// 
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,norun
  /// extern crate animality;
  /// use animality::Animal;
  /// 
  /// let animal: Animal = "cat".parse().unwrap();
  /// ```
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s_lowercased = s.to_lowercase();

    match &s_lowercased[..] {
      "cat" => Ok(Animal::Cat),
      "dog" => Ok(Animal::Dog),
      "bird" => Ok(Animal::Bird),
      "panda" => Ok(Animal::Panda),
      "redpanda" => Ok(Animal::Redpanda),
      "koala" => Ok(Animal::Koala),
      "fox" => Ok(Animal::Fox),
      "whale" => Ok(Animal::Whale),
      "dolphin" => Ok(Animal::Dolphin),
      "kangaroo" => Ok(Animal::Kangaroo),
      "bunny" => Ok(Animal::Bunny),
      "lion" => Ok(Animal::Lion),
      "bear" => Ok(Animal::Bear),
      "frog" => Ok(Animal::Frog),
      "duck" => Ok(Animal::Duck),
      "penguin" => Ok(Animal::Penguin),
      "axolotl" => Ok(Animal::Axolotl),
      "capybara" => Ok(Animal::Capybara),
      _ => Err(InvalidAnimalError),
    }
  }
}

impl Display for Animal {
  #[inline(always)]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}