# animality.rs
A simple API wrapper that generates images & facts of any animal

# Installation
```bash
$ echo :tada:
```

# Example
We use [Tokio](https://tokio.rs) to help with the asynchronous runtime for Rust.

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animality = Animality::new("APIKEY");
    let result = animality.fetch_random_image(String::from("dolphin")).await?;
    let result2 = animality.fetch_fact(String::from("cat")).await?;

    println!("{}", result.link);
    println!("{}", result2.fact);

    Ok(());
}
```

This outputs the following text in the terminal:

```json
{
  "link": "https://api.animality.xyz/images/capybara/20.png",
  "fact": "Capybaras can make for good pets when kept in groups."
}
```
