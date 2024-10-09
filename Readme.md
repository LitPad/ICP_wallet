# Litpad's ICP Wallet Repo

## Commands

1. Run build to install crates and build:

```bash
$ cargo build
```

2. Start a local instance of the Internet Computer locally:

```bash
$ dfx start 
```

3. Create the canister and obtain it's private key:

```bash
$ dfx canister create wallet
```

4. Build the smart contract:

```bash
$ dfx build
```

5. Deploy to the local running ICP:

```bash
$ dfx deploy
```

Note: Install wasm32 using:

```bash
$ rustup target add wasm32-unknown-unknown
```