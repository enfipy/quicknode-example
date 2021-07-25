use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {
    let rpc_url = std::env::var("RPC_URL").unwrap();
    println!("{:?}", rpc_url);
    let rpc_client = Client::new(rpc_url, Auth::None).unwrap();
    let info = rpc_client.get_blockchain_info().unwrap();
    println!("Info: {:?}", info);
}
