check:
    cargo check --workspace && cargo check --target wasm32-unknown-unknown

meta-merge:
    git merge -s ours --no-ff upstream/main -m "Meta-merge upstream/main using -s ours (keep fork content unchanged)"