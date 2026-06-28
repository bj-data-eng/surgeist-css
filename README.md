# surgeist-css

Strict CSS ingestion for Surgeist. This crate parses CSS-facing input and lowers it into typed style data owned by `surgeist-style`.

## API Artifact

The committed API coordination artifact lives at `api/public-api.txt`, but the
generator is owned by the root `surgeist` repo.

Refresh this crate's artifact from the root repo with:

```sh
cargo run --manifest-path api/generator/Cargo.toml -- --crate surgeist-css
```

API refresh tooling is command-only and must not run as part of normal `cargo test`.
