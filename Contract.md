
# Neurolov Presale Contract Documentation


## 1. Introduction

### 1.1 Project Overview
The Neurolov presale contract is designed to facilitate a presale for the Neurolov (NLOV) token on the Solana blockchain. The contract allows users to contribute SOL to participate in the presale and claim their tokens once the public sale ends. It also includes features for pausing, unpausing, finalizing the presale, and withdrawing funds.

### 1.2 Technology Stack
- **Blockchain**: Solana
- **Framework**: Anchor
- **Programming Language**: Rust

## 2. Contract Design

### 2.1 Module Overview
The `neurolov_presale` module includes functionalities for:
- Initializing the presale
- Allowing user contributions
- Claiming tokens
- Pausing/unpausing the presale
- Finalizing the presale
- Withdrawing funds

### 2.2 Key Components
- **Presale**: Manages the presale state, token transfers, and contributions.
- **UserInfo**: Tracks individual user contributions and claims.
- **Events**: Tracks important contract activities such as initialization, contributions, claims, and pauses.

## 3. Functions

### 3.1 `initialize`
- **Purpose**: Initializes the presale parameters.
- **Inputs**:
  - `start_time`: The start time of the presale (UNIX timestamp).
  - `end_time`: The end time of the presale (UNIX timestamp).
  - `token_amount`: The total amount of tokens allocated for the presale.
  - `token_price`: The price of each token in lamports.
- **Outputs**: None
- **Errors**:
  - `InvalidTimeRange`: When the start time is not before the end time.

### 3.2 `contribute`
- **Purpose**: Allows users to contribute SOL to the presale.
- **Inputs**:
  - `amount`: The amount of SOL to contribute.
- **Outputs**: None
- **Errors**:
  - `PresalePaused`: When the presale is paused.
  - `PresaleNotActive`: When the presale is not active.
  - `InvalidAmount`: When the contribution amount is invalid or exceeds the available supply.

### 3.3 `claim_tokens`
- **Purpose**: Allows users to claim their tokens after the public sale ends.
- **Inputs**: None
- **Outputs**: None
- **Errors**:
  - `PresalePaused`: When the presale is paused.
  - `ClaimingNotAvailable`: When the claiming period is not yet available.
  - `NothingToClaim`: When the user has no tokens to claim.

### 3.4 `pause`
- **Purpose**: Pauses the presale.
- **Inputs**: None
- **Outputs**: None
- **Errors**:
  - `Unauthorized`: When the caller is not the owner.
  - `AlreadyPaused`: When the presale is already paused.

### 3.5 `unpause`
- **Purpose**: Unpauses the presale.
- **Inputs**: None
- **Outputs**: None
- **Errors**:
  - `Unauthorized`: When the caller is not the owner.
  - `NotPaused`: When the presale is not paused.

### 3.6 `finalize_presale`
- **Purpose**: Finalizes the presale after the end time.
- **Inputs**: None
- **Outputs**: None
- **Errors**:
  - `Unauthorized`: When the caller is not the owner.
  - `PresaleStillActive`: When the presale is still active.
  - `PresaleAlreadyFinalized`: When the presale is already finalized.

### 3.7 `withdraw`
- **Purpose**: Withdraws SOL from the presale account to the owner.
- **Inputs**:
  - `amount`: The amount of SOL to withdraw.
- **Outputs**: None
- **Errors**:
  - `Unauthorized`: When the caller is not the owner.
  - `PresaleStillActive`: When the presale is still active.
  - `InsufficientFunds`: When there are not enough funds in the presale account.

## 4. Account Structures

### 4.1 Presale
- **Description**: Stores the presale parameters and state.
- **Fields**:
  - `start_time`: UNIX timestamp for presale start time.
  - `end_time`: UNIX timestamp for presale end time.
  - `public_sale_end_time`: UNIX timestamp for the end of the public sale.
  - `token_mint`: Public key of the token mint.
  - `presale_token_account`: Public key of the presale token account.
  - `presale_supply`: Total supply of tokens for the presale.
  - `token_price`: Price of the token in lamports.
  - `total_contributed`: Total amount of SOL contributed.
  - `is_active`: Boolean flag indicating if the presale is active.
  - `is_paused`: Boolean flag indicating if the presale is paused.
  - `owner`: Public key of the presale owner.

### 4.2 UserInfo
- **Description**: Stores information about user contributions and claims.
- **Fields**:
  - `user`: Public key of the user.
  - `amount_contributed`: Total amount of SOL contributed by the user.
  - `amount_claimed`: Total amount of tokens claimed by the user.

## 5. Events

### 5.1 `PresaleInitialized`
- **Fields**:
  - `start_time`: UNIX timestamp for presale start time.
  - `end_time`: UNIX timestamp for presale end time.
  - `token_amount`: Total tokens allocated for the presale.
  - `token_price`: Price of each token in lamports.

### 5.2 `ContributionMade`
- **Fields**:
  - `user`: Public key of the user making the contribution.
  - `sol_amount`: Amount of SOL contributed.
  - `nlov_amount`: Amount of NLOV tokens contributed.
  - `total_contributed`: Total amount of SOL contributed in the presale.

### 5.3 `TokensClaimed`
- **Fields**:
  - `user`: Public key of the user claiming tokens.
  - `amount`: Amount of tokens claimed.

### 5.4 `PresalePaused`
- **Fields**: None

### 5.5 `PresaleUnpaused`
- **Fields**: None

### 5.6 `PresaleFinalized`
- **Fields**:
  - `total_contributed`: Total amount of SOL contributed in the presale.
  - `end_time`: UNIX timestamp for the presale end time.

### 5.7 `FundsWithdrawn`
- **Fields**:
  - `owner`: Public key of the owner withdrawing funds.
  - `amount`: Amount of SOL withdrawn.

## 6. Error Codes

### 6.1 `PresaleNotActive`
- **Message**: "Presale is not active."

### 6.2 `InvalidAmount`
- **Message**: "Amount is invalid."

### 6.3 `Unauthorized`
- **Message**: "Unauthorized."

### 6.4 `InvalidTimeRange`
- **Message**: "Invalid start and end times."

### 6.5 `ExceedsPresaleSupply`
- **Message**: "Presale supply exceeded."

### 6.6 `ClaimingNotAvailable`
- **Message**: "Claiming is not available yet."

### 6.7 `NothingToClaim`
- **Message**: "Nothing to claim."

### 6.8 `PythError`
- **Message**: "Error fetching Pyth price."

### 6.9 `InvalidPythPrice`
- **Message**: "Invalid Pyth price."

### 6.10 `PresalePaused`
- **Message**: "Presale is paused."

### 6.11 `AlreadyPaused`
- **Message**: "Presale is already paused."

### 6.12 `NotPaused`
- **Message**: "Presale is not paused."

### 6.13 `PresaleStillActive`
- **Message**: "Presale is still active."

### 6.14 `PresaleAlreadyFinalized`
- **Message**: "Presale has already been finalized."

### 6.15 `InsufficientFunds`
- **Message**: "Insufficient funds."

### 6.16 `CalculationError`
- **Message**: "Calculation error."

## 7. Testing

### 7.1 Overview
The contract should be thoroughly tested to ensure all functions perform as expected and handle errors correctly. Testing should include:
- **Unit Tests**: Test individual functions in isolation to ensure they work correctly.
- **Integration Tests**: Test the complete workflow from initialization to finalization and claiming.

### 7.2 Testing Strategy
- **Unit Tests**: Verify the correctness of individual functions.
- **Integration Tests**: Ensure the overall functionality and integration of the contract.

## 8. Deployment

### 8.1 Deployment Instructions
1. **Compile** the contract using Anchor.
2. **Deploy** the contract to the Solana network using the Anchor CLI.
3. **Configure** any required parameters or settings post-deployment.

### 8.2 Configuration
- Ensure that `token_mint` and `presale_token_account` are correctly set up before initializing the presale.

## 9. Appendix

### 9.1 References
- [Anchor Framework](https://project-serum.github.io/anchor/)
- [Solana Documentation](https://docs.solana.com/)

### 9.2 Glossary
- **SOL**: Solana's native cryptocurrency.
- **NLOV**: The Neurolov token.
- **Lamports**: The smallest unit of SOL.

