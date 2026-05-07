# Databricks Integration Tests

The demo code that previously lived inside `main()` has been promoted to async
unit tests in `src/test/databrick_table.rs`, wired into the binary via the
`#[cfg(test)] mod test;` declaration in `src/main.rs`. Each test exercises a
real Databricks SQL Warehouse, so they are marked `#[ignore]` and only run
when explicitly requested.

## Prerequisites

1. Make sure `.env` at the project root contains valid Databricks credentials:

   ```env
   DATABRICKS_BASE_URL=https://<your-workspace>.azuredatabricks.net
   DATABRICKS_TOKEN=dapiXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
   DATABRICKS_WAREHOUSE_ID=1c3b895a4bf16947
   DATABRICKS_WAIT_TIMEOUT=30s
   ```

2. The target table used by the SQL tests is hard-coded in
   `src/test/databrick_table.rs`:

   ```rust
   const TABLE: &str = "amigo_dv.brnz_sd.diamonds";
   ```

   It must exist in your workspace with at least the columns
   `clarity`, `color`, `cut`, `carat`.

## Available Tests

| Test                          | What it does                                          |
| ----------------------------- | ----------------------------------------------------- |
| `get_warehouses_returns_list` | `GET /api/2.0/sql/warehouses` and asserts non-empty   |
| `insert_diamond_row`          | INSERT a sample diamond row                           |
| `update_diamond_row`          | UPDATE color of rows where `clarity = 'SI2'`          |
| `delete_diamond_row`          | DELETE rows where `clarity = 'SI3'`                   |
| `select_diamond_rows`         | SELECT * LIMIT 100 and pretty-prints the JSON result  |

## How to Run

All Databricks tests are gated behind `#[ignore]`. A normal `cargo test` run
will skip them.

### Run every ignored test

```bash
cargo test -- --ignored --nocapture
```

`--nocapture` keeps the `println!` output (useful for inspecting the warehouse
list, generated SQL, and SELECT JSON).

### Run a single test

`cargo test` accepts a substring filter, so any of the names below work:

```bash
cargo test test::databrick_table::get_warehouses_returns_list -- --ignored --nocapture
cargo test test::databrick_table::insert_diamond_row          -- --ignored --nocapture
cargo test test::databrick_table::update_diamond_row          -- --ignored --nocapture
cargo test test::databrick_table::delete_diamond_row          -- --ignored --nocapture
cargo test test::databrick_table::select_diamond_rows         -- --ignored --nocapture
```

You can also use the short name (e.g. `cargo test insert_diamond_row -- --ignored`).

### Run only the `databrick_table` module

```bash
cargo test test::databrick_table -- --ignored --nocapture
```

### Run them in order (recommended)

The mutation tests are independent, but a clean smoke run looks like:

```bash
cargo test get_warehouses_returns_list -- --ignored --nocapture && \
cargo test insert_diamond_row          -- --ignored --nocapture && \
cargo test select_diamond_rows         -- --ignored --nocapture && \
cargo test update_diamond_row          -- --ignored --nocapture && \
cargo test delete_diamond_row          -- --ignored --nocapture
```

By default `cargo test` runs tests in parallel. To force serial execution
(safer for tests that mutate the same rows), add `--test-threads=1`:

```bash
cargo test -- --ignored --nocapture --test-threads=1
```

## Notes

- The tests panic on API errors via `.expect(..)`, so a failure surfaces the
  underlying `reqwest::Error` message in the test output.
- `get_warehouses_returns_list` only requires `DATABRICKS_BASE_URL` and
  `DATABRICKS_TOKEN`; the SQL tests additionally require
  `DATABRICKS_WAREHOUSE_ID` and `DATABRICKS_WAIT_TIMEOUT`.
- `main()` itself is now just a placeholder that prints a hint to run the
  tests; the binary no longer performs any network calls on `cargo run`.
