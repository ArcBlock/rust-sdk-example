extern crate forge_grpc;
extern crate forge_wallet;

use forge_grpc::{connection, transaction, Result};

/// Example:
/// - 1.add connection with forge chain
/// - 2.create local wallet
/// - 3.declare wallet on forge chain
/// - 4.checkin to some tokens
/// - 5.transfer some tokens to other
/// - 6. get account state
fn transfer() -> Result<()> {
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
    let decimal = connection::get_connection(Some(chain_name.to_string()))
        .unwrap()
        .get_decimal() as usize;
    let transfer_itx = transaction::build_itx::Transfer {
        to: Some(bob.address.to_owned()),
        value: Some(forge_grpc::BigUint::from_string("1", decimal)?),
        ..Default::default()
    };
    let resp = forge_grpc::transfer(&request, &transfer_itx)?;
    println!("transfer, resp {:?}", resp);
    assert!(!resp.get_hash().is_empty());

    // -8.if the tx stable, then get balance.
    let hash = resp.get_hash().to_owned();
    let tx_resp = forge_grpc::check_tx(&vec![hash.to_owned()], None).unwrap();
    assert!(tx_resp.get(&hash).unwrap());

    // -9.get balance
    let resp = forge_grpc::get_account_state(
        &vec![alice.address, bob.address],
        Some(chain_name.to_string()),
    )
    .unwrap();
    println!(
        "alice balance: {:#?}, bob balance: {:#?}",
        resp[0].get_state().get_balance().to_string(decimal),
        resp[1].get_state().get_balance().to_string(decimal)
    );

    Ok(())
}

fn main() -> Result<()> {
    transfer()?;
    Ok(())
}
