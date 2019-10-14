extern crate forge_grpc;
// extern crate forge_sdk;
extern crate forge_wallet;

use forge_grpc::{connection, transaction, Result};

fn get_forge_info() -> Result<()> {
    let chain_address = "127.0.0.1:28210";
    let chain_name = "chain_1";

    // -1.create connection with forge chain
    connection::add_connection(chain_name, chain_address)?;

    // -2.get chain info
    let chain_info = forge_grpc::get_chain_info(Some(chain_name.to_string()))?;
    println!("chain_info : {:#?}", chain_info);

    // -3.create two wallets: alice, bob
    let alice = forge_wallet::Wallet::create_default_wallet()?;
    let bob = forge_wallet::Wallet::create_default_wallet()?;

    // -4.declare alice on chain
    let mut request = transaction::build_request::Request {
        wallet: alice.clone(),
        forge_name: Some(chain_name.to_string()),
        ..Default::default()
    };
    let mut declare = transaction::build_itx::Declare {
        moniker: Some(String::from("alice")),
        ..Default::default()
    };
    let resp = forge_grpc::declare(&request, &declare)?;
    println!("alice declare , resp {:?}", resp);
    assert!(!resp.get_hash().is_empty());

    // -5.declare bob on chain
    request.wallet = bob.clone();
    declare.moniker = Some(String::from("bob_01"));
    let resp = forge_grpc::declare(&request, &declare)?;
    println!("bob declare , resp {:?}", resp);
    assert!(!resp.get_hash().is_empty());

    // -6.alice checkin to get some token: default 25 token.
    request.wallet = alice.clone();
    let resp = forge_grpc::poke(&request)?;
    println!("alice checkin, resp {:?}", resp);
    assert!(!resp.get_hash().is_empty());

    // -7.alice transfer 1 token to bob
    let transfer_itx = transaction::build_itx::Transfer {
        to: Some(bob.address.to_owned()),
        value: Some(1.0),
        ..Default::default()
    };
    let resp = forge_grpc::transfer(&request, &transfer_itx)?;
    println!("transfer, resp {:?}", resp);
    assert!(!resp.get_hash().is_empty());

    Ok(())
}

fn main() -> Result<()> {
    get_forge_info()?;
    Ok(())
}
