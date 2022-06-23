# animality.rs
A simple Rust API wrapper that generates images & facts of any animal.

# Installation
Add this to your `Cargo.toml file's dependencies:`
```toml
animality = "1.0.0"
```

# Blocking Request
```rust
extern crate animality;
use animality::{Animality, Animal};

fn main() {
  let client = Animality::new("your token here");
  
  // request with the `Animal` enum
  let dog_image = client.image(Animal::Dog).unwrap();
  let dog_fact = client.fact(Animal::Dog).unwrap();
  
  // request from a string (case-insensitive) 
  let cat: Animal = "cat".parse().unwrap();
  let cat_image = client.image(cat).unwrap();
  let cat_fact = client.fact(cat).unwrap();
}
```

# Async Request
```rust
extern crate animality;
extern crate tokio;

use animality::{Animality, Animal, RequestError};

#[tokio::main]
async fn main() -> Result<(), RequestError> {
  let client = Animality::new("your token here");
  
  // request with the `Animal` enum
  let dog_image = client.image_async(Animal::Dog).await?;
  let dog_fact = client.fact_async(Animal::Dog).await?;
  
  // request from a string (case-insensitive) 
  let cat: Animal = "cat".parse().unwrap();
  let cat_image = client.image_async(cat).await?;
  let cat_fact = client.fact_async(cat).await?;

  Ok(())
}
```
