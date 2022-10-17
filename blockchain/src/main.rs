use blockchain::crypto;

use colored::*;
use std::fmt::Write;
use std::fs::File;
use std::future::Future;
use std::net::SocketAddr;
use jsonrpsee::{rpc_params, RpcModule};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::WsServerBuilder;
use blockchain::crypto::utils::generate_random_keypair;
use blockchain::primitives::address::Address;


fn to_hex(data: &[u32; 8]) -> String {
    let mut hex = String::new();

    for &b in data {
        write!(hex, "{:08x}", b.to_le()).unwrap();
    }

    hex
}

async fn start_block_chain() -> anyhow::Result<()> {
    println!("{}", "[INFO] Starting node...".green());
    let server = WsServerBuilder::default().build("127.0.0.1:8283").await?;
    let mut module = RpcModule::new(());
    blockchain::rpc::methods::register_methods(&mut module);
    server.start(module)?;
    println!("{}", "[INFO] Node Started".green());

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // start_block_chain().await?;
    println!("{}", Address::new());
    // keep running
    loop {}
}
