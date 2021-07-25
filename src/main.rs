use bdk::bitcoincore_rpc::{json::EstimateMode, Auth, Client, RpcApi};

fn main() {
    let rpc_url = std::env::var("RPC_URL").unwrap();
    println!("{:?}", rpc_url);
    let rpc_client = Client::new(rpc_url, Auth::None).unwrap();
    let fee = rpc_client
        .estimate_smart_fee(1, Some(EstimateMode::Economical))
        .unwrap();
    println!("{:?}", fee);
}
