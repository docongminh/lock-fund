# Lock Fund Program
A Escrow program with multi-signs and accompanying scripts (TypeScript & CLI) designed to provide safer ways to lock/hold fund on the Solana blockchain. Adds an extra layer of security for your tokens.

## Purpose
  - Enhanced Security: Lock funds with multiple signers instead of a single private key. And all your private saved under encrypted with password.
  - Use Case:
    - You hold a large amount of tokens and more safe with your fund.
    - Lock your fund with cliff amount of time to become diamond hand

### Build & Deploy the Program
Below are general steps to build and deploy the program to Solana

Build program
```bash
anchor build
```

Deploy program
```bash
anchor deploy
```

### Cli
You can use cli to easily interact with the escrow program without writing code.

Build Cli
```bash
cd cli && cargo build
```

Cli package supported commands:
```bash
./target/debug/cli --help
```

### TODO
  - Encrypt private then save only encrypted private
  - Lock fund with cliff time
  - admin instructions
  - unit test
