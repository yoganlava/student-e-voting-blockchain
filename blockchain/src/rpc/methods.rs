use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule<()>) {
    module.register_method(
        "process_transaction", |_,_| Ok("Pong")
    );

    module.register_method(
        "get_last_block", |_,_| Ok("Pong")
    );

    module.register_method(
        "generate_random_keypair", |_,_| Ok("Pong")
    );
}