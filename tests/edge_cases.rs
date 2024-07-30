#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use solana_program::instruction::Instruction;
    use solana_program::transaction::Transaction;
    use anchor_lang::ProgramTest;

    #[tokio::test]
    async fn test_edge_cases() {
        let program_id = Pubkey::new_unique();
        let (presale_account, _) = Pubkey::find_program_address(&[b"presale"], &program_id);
        let user_account = Pubkey::new_unique();
        let payer = Pubkey::new_unique();

        let mut program = ProgramTest::new(
            "nlov",
            program_id,
            processor!(process_instruction),
        );

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

        let (mut banks_client, payer, recent_blockhash) = program.start().await;

        // Test with extreme values
        let ix = Instruction::new_with_bincode(
            program_id,
            &PresaleInstruction::Contribute { amount: u64::MAX },
            vec![AccountMeta::new(user_account, false), AccountMeta::new(presale_account, false)],
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Expected transaction to fail with extreme value");
    }
}
