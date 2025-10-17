use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    signer::keypair::Keypair,
    transaction::Transaction,
};

#[tokio::test]
async fn cross_program_invocation_program_test() {
    let called_program_id = Pubkey::new_unique();
    let caller_program_id = Pubkey::new_unique();

    let mut program_test = ProgramTest::default();

    program_test.add_program(
        "called",
        called_program_id,
        processor!(called::process_instruction),
    );
    program_test.add_program(
        "caller",
        caller_program_id,
        processor!(caller::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut ix_data = Vec::with_capacity(32);
    ix_data.extend_from_slice(called_program_id.as_ref());

    let instruction = Instruction::new_with_bytes(
        caller_program_id,
        &ix_data,
        vec![AccountMeta::new_readonly(called_program_id, false)],
    );

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client
        .process_transaction(tx)
        .await
        .expect("process_transaction failed");

    println!("Transaction processed by ProgramTest (2.2.0). Run tests with --nocapture to see program logs.");
}
