# Oxidized Hackathon Kit

## Tech Stack
- [Axum](https://github.com/tokio-rs/axum)
- [MongoDB](https://www.mongodb.com)
- [Svelte.js](https://www.svelte.dev)

## Getting Stated

### Prerequisites

- Make sure you have rust and bun installed.

- Make sure you have mongodb compass (local) or atlas (cloud) spinning.

### Setting up

1. Clone the repo

    ```bash
    git clone https://github.com/vilayat-ali/oxidized-hack-kit.git hack
    ```

2. CD into the target folder

    ```bash
    cd hack
    ```

3. Install deps and run

    ```bash
    cd backend
    cargo b --release
    cargo r --release
    ```

    ```bash
    cd ../frontend
    bun install
    bun run dev
    ```

## CONTRIBUTING

Any PRs, suggestions or dicussions are warmly welcomed.

## LICENSE

This project is distributed under [MIT](./LICENSE) license.