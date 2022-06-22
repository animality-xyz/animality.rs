# Animality.rs
> Animality.rs is a rust wrapper for the [Animality](https://animality.xyz) api.
# Example
> We use [tokio](https://tokio.rs) to help with the asynchronous runtime for Rust.
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let animality = Animality::new("APIKEY");
    let result = animality.fetch_random_image(String::from("dolphin")).await?;
    let result2 = animality.fetch_fact(String::from("cat")).await?;

    println!("{}", result.link);
    println!("{}", result2.fact);

    Ok(())
}
```
> Output:

```json
{
  "link": "https://api.animality.xyz/images/dolphin/20.png",
  "fact": "Black cats are bad luck in the United States, but they are good luck in the United Kingdom and Australia."
}
```