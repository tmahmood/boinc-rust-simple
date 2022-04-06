use tokio;
use boinc_rpc;

#[tokio::main]
async fn main() {
    let transport =
        boinc_rpc::Transport::new("127.0.0.1:31416", Some("ae6ae835752ca8a5b45ee05cea519541"));
    let mut client = boinc_rpc::Client::new(transport);

    println!("{:?}\n", client.get_account_manager_info().await.unwrap());
    println!("{:?}\n", client.get_account_manager_info().await.unwrap());
}
