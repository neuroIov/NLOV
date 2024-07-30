#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
    use anchor_lang::solana_program::transaction::Transaction;
    use anchor_lang::ProgramTest;

    #[tokio::test]
    async fn test_reentrancy_attack() {
        // Initialize the program
        let program_id = Pubkey::new_unique();
        let (presale_account, _) = Pubkey::find_program_address(&[b"presale"], &program_id);
        let attacker_account = Pubkey::new_unique();
        let user_account = Pubkey::new_unique();
        let payer = Keypair::new();

        let mut program = ProgramTest::new(
            "nlov", // program 
            program_id,
            processor!(process_instruction),
        );

        // Add accounts
        program.add_account(
            presale_account,
            AccountDescriptor {
                lamports: 1_000_000,
                data: vec![0; Presale::LEN],
                owner: program_id,
            },
        );

        program.add_account(
            user_account,
            AccountDescriptor {
                lamports: 1_000_000,
                data: vec![0; UserInfo::LEN],
                owner: program_id,
            },
        );

        program.add_account(
            attacker_account,
            AccountDescriptor {
                lamports: 1_000_000,
                data: vec![0; UserInfo::LEN],
                owner: program_id,
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program.start().await;

        // Create an attacker account with malicious logic
        let malicious_program_id = Pubkey::new_unique();
        let malicious_ix = Instruction::new_with_bincode(
            malicious_program_id,
            &MaliciousInstruction::Attack { /* parameters */ },
            vec![
                AccountMeta::new(attacker_account, false),
                AccountMeta::new(presale_account, false),
            ],
        );

        // Try to exploit the contract
        let attack_tx = Transaction::new_signed_with_payer(
            &[malicious_ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(attack_tx).await;
        assert!(result.is_err(), "Expected attack transaction to fail");

        // Verify that the presale contract state is unchanged and secure
      
        // Add specific or more assertions depending on  contract's state and expected behavior
    }
}
