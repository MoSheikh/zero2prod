# zero2prod
Code following the "Zero To Production in Rust" book

## Migrations

### Requirements

Run `cargo install diesel_cli --no-default-features --features postgres` to be able to run migrations via the CLI for development and one-off testing.

## Configuration

### Environment Variables

- `TEST_LOG`: set to `true` to enable server logging for integration tests.
- `APP_ENV`: select from either `local` or `production` to load the appropriate configuration.
