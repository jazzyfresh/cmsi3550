mod network;
use std::env::args;

#[tokio::main]
async fn main() {
    // must pass 2 args w/ localhost:<port>
    let my_args: Vec<String> = args().collect();
    let local_addr = my_args[1].to_string();
    let remote_addr = my_args[2].to_string();

    // Start the network listener
    let listener_task = tokio::spawn(async move {
        network::start_network_listener(&local_addr).await.unwrap();
    });

    // Start the network connector
    let connector_task = tokio::spawn(async move {
        network::start_network_connector(&remote_addr)
            .await
            .unwrap();
    });

    // Await both tasks
    let _ = tokio::try_join!(listener_task, connector_task);
}
