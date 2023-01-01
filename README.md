## Overview

Purpose: Compare structure data captured by DWARF, and CodeQL.

Motivation: https://github.com/github/codeql/issues/11790, getting better at rust

Heavily based on the examples in gimli

## Usage

First, generate a CodeQL database, and executable with DWARF debug information.
Make sure they have the same configuration!

Then, run the following CodeQL query and get a JSON representation of its output

#### Get JSON
```
codeql query run ./codeql/query.ql -d ./codeql_db -o codeql_res.bqrs
codeql bqrs decode --format=json codeql.bqrs -o codeql.json 
```

#### Pass to Struct_Check
```
cargo run exe_path_with_dwarf codeql.json
```
