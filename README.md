# animality.rs
A simple API wrapper that generates images & facts of any animal

# Installation
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
animality = { version = "0.1.0" }
```

# Example
We use [Tokio](https://tokio.rs) to help with the asynchronous runtime for Rust.

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animality = Animality::new("API_KEY");
    let image = animality.fetch_random_image(String::from("capybara")).await?;
    let fact = animality.fetch_fact(String::from("capybara")).await?;

    println!("{}", image.link);
    println!("{}", fact.fact);

    Ok(());
};
```

This outputs the following text in the terminal:

```json
{
  "link": "https://api.animality.xyz/images/capybara/20.png",
  "fact": "Capybaras can make for good pets when kept in groups."
}
```
