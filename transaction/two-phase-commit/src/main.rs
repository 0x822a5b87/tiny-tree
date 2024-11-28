use std::future::Future;

mod transaction;

// Boilerplate which lets us write `async fn main`, we'll explain it later.
#[tokio::main]
async fn main() {
    let (sender, receiver) = async_channel::unbounded();
    let coordinator = &mut transaction::Coordinator::new(10, &sender, &receiver);
    coordinator.transaction();

    while !coordinator.transaction_end() {
        for x in &coordinator.participants {
        }
    }
}
