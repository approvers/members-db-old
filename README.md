# members-db
Database of members in Approvers, implemented in Rust.

## Usage
```console
$ cargo run [options] <command> [args...]
```

## Commands
- **serve**  
  Starts the HTTP server to serve the members as JSON.

- **list**  
  Lists all members.

- **add \<name\>**  
  Adds a member with the name.
  
- **migrate**  
  Migrates the database to the current version.

## Options
 - **-d \<path\>, --database=\<path\>**  
   Forces to use the database located at the path.
