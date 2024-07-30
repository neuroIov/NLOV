use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::clock::Clock;
use solana_program_test::*;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;

use my_presale_project::{self, Presale, UserInfo, ErrorCode};

#[tokio::test]
async fn test_initialize() {
    let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}

#[tokio::test]
async fn test_contribute() {
     let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}

#[tokio::test]
async fn test_pause_presale() {
     let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}

#[tokio::test]
async fn test_unpause_presale() {
       let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}

#[tokio::test]
async fn test_finalize_presale() {
      let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}

#[tokio::test]
async fn test_claim_tokens() {
       let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}

#[tokio::test]
async fn test_withdraw_funds() {
       let mut program_test = ProgramTest::new(
        "my_presale_project", // Run the BPF version of the program
        my_presale_project::ID,
        processor!(my_presale_project::entry),
    );

    // Add presale account
    let presale_account = Keypair::new();
    program_test.add_account(
        presale_account.pubkey(),
        Account {
            lamports: 1_000_000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let token_mint = Keypair::new();
    let owner_token_account = Keypair::new();
    let presale_token_account = Keypair::new();

    let start_time = Clock::get().unwrap().unix_timestamp;
    let end_time = start_time + 86400; // 24 hours later
    let token_amount = 1_000_000;
    let token_price = 1_000; // 1 SOL per token

    let transaction = Transaction::new_signed_with_payer(
        &[my_presale_project::instruction::initialize(
            &my_presale_project::ID,
            start_time,
            end_time,
            token_amount,
            token_price,
            &presale_account.pubkey(),
            &token_mint.pubkey(),
            &presale_token_account.pubkey(),
            &owner_token_account.pubkey(),
            &payer.pubkey(),
        )],
        Some(&payer.pubkey()),
        &[&payer, &presale_account],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    let presale_account_data = banks_client
        .get_account_data_with_borsh::<Presale>(presale_account.pubkey())
        .await
        .unwrap();

    assert_eq!(presale_account_data.start_time, start_time);
    assert_eq!(presale_account_data.end_time, end_time);
    assert_eq!(presale_account_data.presale_supply, token_amount);
    assert_eq!(presale_account_data.token_price, token_price);
    assert_eq!(presale_account_data.total_contributed, 0);
    assert_eq!(presale_account_data.is_active, true);
    assert_eq!(presale_account_data.is_paused, false);
}
