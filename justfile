check:
    cargo check --workspace --all-features && cargo check --target wasm32-unknown-unknown --all-features

meta-merge:
    git merge -s ours --no-ff upstream/main -m "Meta-merge upstream/main using -s ours (keep fork content unchanged)"