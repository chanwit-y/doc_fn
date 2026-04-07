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