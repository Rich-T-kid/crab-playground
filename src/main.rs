#[tokio::main]
async fn main() {
    let x = add(20, 40).await;
    println!("{x}")
}

async fn add(a: u8, b: u8) -> u8 {
    a + b
}
