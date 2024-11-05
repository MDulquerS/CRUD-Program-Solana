# Solana CRUD Program with Anchor

This program demonstrates a simple CRUD (Create, Read, Update, Delete) system on the Solana blockchain using the Anchor framework. It allows users to create, update, and delete journal entries that are stored on-chain.

## Features

- **Create a Journal Entry**: Users can create journal entries with a title and message.
- **Update a Journal Entry**: Users can update the message in an existing journal entry.
- **Delete a Journal Entry**: Users can delete their journal entry.

## Program Structure

### Entry Functions

- `create_journal_state`: Initializes a new journal entry account with an owner, title, and message.
- `update_journal_entry`: Updates the message of an existing journal entry.
- `delete_journal_entry`: Deletes a specified journal entry and closes the associated account.

### PDA (Program Derived Address)

Each journal entry account is assigned a unique PDA based on the `title` and the `owner`'s public key. This ensures that each journal entry is unique and associated with the creator.

## Accounts

### `CreateEntry`

- Initializes a new journal entry with an owner, title, and message.
- Uses a PDA derived from the `title` and `owner` public key to uniquely identify each entry.
- Allocates space according to the `JournalEntryState` structure.

### `UpdateEntry`

- Updates an existing journal entry by modifying its `message`.
- Supports account reallocation to handle potential changes in the data length.

### `DeleteEntry`

- Closes an existing journal entry and returns the rent to the owner.

## Journal Entry State

The `JournalEntryState` struct defines the on-chain data structure for each entry:

- **Owner**: Public key of the account owner.
- **Title**: Title of the journal entry (maximum 50 characters).
- **Message**: Content of the journal entry (maximum 1000 characters).

## Usage

### Prerequisites

- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/introduction.html)
- Solana CLI installed

### Building and Deploying

1. **Build the program**:

   ```bash
   anchor build
   ```

2. **Deploy the program**:
   ```bash
   anchor deploy
   ```

### Running Tests

Anchor tests can be run to verify the functionality of the program:

```bash
anchor test
```
