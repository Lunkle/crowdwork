use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    example_async_function().await;
}

async fn example_async_function() {
    println!("This is an example async function.");
}
