use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{self, Instruction, AccountMeta},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::{self, Transaction},
};
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentLevel;
use std::{thread, time::Duration};



#[tokio::test(flavor = "multi_thread", worker_threads = 2)]

async fn cross_programm_invocation(){
    //change variables to deployed programm ids
    let program_caller_id = Pubkey::from_str("4Dnzf3hvFrrPQddkgVd1nYynLRwJtc4RCQa6jpjauryN").unwrap();
    let program_called_id = Pubkey::from_str("B2PLCtHuaS8umkVMJaNhApjUVa2fTzZR9vcvBqnVcnBp").unwrap();

    let rpc_url = "http://127.0.0.1:8899";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = Keypair::new();
    let sign = client.request_airdrop(&payer.pubkey(), 2_000_000_000).unwrap();
    loop {
        let conf = client.confirm_transaction(&sign).unwrap();
        if conf {
            break;
        }
    }

    let mut ix_data = vec![];
    ix_data.extend_from_slice(program_called_id.as_ref());
    
    let instruction = Instruction::new_with_bytes(program_caller_id, &ix_data, vec![AccountMeta::new_readonly(program_called_id, false),]);
    let message = Message::new(&[instruction],Some(&payer.pubkey()));
    let last_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new(&[&payer], message, last_blockhash);

    let signature = client
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::confirmed(),
            RpcSendTransactionConfig {
                skip_preflight: false,
                preflight_commitment: Some(CommitmentConfig::processed().commitment),
                ..RpcSendTransactionConfig::default()
            },
        )
        .expect("Transaction failed");

    println!("Transaction send:{}" , signature);


let tx_config = RpcTransactionConfig {
    encoding: Some(UiTransactionEncoding::Json),
    commitment: Some(CommitmentConfig {
        commitment: CommitmentLevel::Finalized,
    }),
    max_supported_transaction_version: None,
};

let confirmed = loop {
    match client.get_transaction_with_config(
        &signature,
        RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(CommitmentConfig { commitment: CommitmentLevel::Finalized }),
            max_supported_transaction_version: None,
        },
    ) {
        Ok(tx) => break tx, // this gives EncodedConfirmedTransactionWithStatusMeta
        Err(err) => {
            if err.to_string().contains("Transaction not found") || err.to_string().contains("null") {
                thread::sleep(Duration::from_millis(500)); // wait a bit and retry
            } else {
                panic!("RPC error: {:?}", err);
            }
        }
    }
};

    let meta = confirmed.transaction.meta.expect("No meta in transaction");
    let logs = meta.log_messages.expect("No logs in transaction");

    println!("Logs:");
    for log in &logs{
        println!("{}",log);
    }

    let caller_recive = logs.iter().any(|l| l.contains("Hitted caller!"));
    let called_invoke = logs.iter().any(|l| l.contains("Caller succesfully invoked called"));

    assert!(caller_recive, "Caller did not execute correctly");
    assert!(called_invoke, "Caller was not invoked");

}