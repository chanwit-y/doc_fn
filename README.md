```sh
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

task
- config logic app
- listener
- move file and insert file info into database
- databricks function
   - upload file
   - get data from table
- check file info for merge
- merge file function
- upload file to ecm


You can run just the tests in `src/test/http.rs` by specifying the module path:

```bash
cargo test test::http
```

This will only run test functions inside the `test::http` module. If you want a single specific test:

```bash
cargo test test::http::test_azure_sp_auth
```