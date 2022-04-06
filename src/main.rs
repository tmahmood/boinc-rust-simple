use tokio;
use boinc_rpc;

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let transport =
            boinc_rpc::Transport::new("127.0.0.1:31416", Some("c8e3b1521006593472c1078dc4f8d76e"));
        let mut client = boinc_rpc::Client::new(transport);

        println!("{:?}\n", client.get_account_manager_info().await.unwrap());
        println!("{:?}\n", client.get_account_manager_info().await.unwrap());
    })
}
