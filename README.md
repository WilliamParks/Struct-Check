## Overview

Purpose: Compare structure data captured by DWARF, and CodeQL.

Motivation: https://github.com/github/codeql/issues/11790, getting better at rust

Heavily based on the examples in gimli

## Usage

First, generate a CodeQL database, and executable with DWARF debug information.
Make sure they have the same configuration!

Then, run the following CodeQL query and get a JSON representation of its output

#### Query
```
import cpp

from Class c
select c.getName(), c.getSize()
```

#### Get JSON
```
codeql query run ./temp.ql -d ./codeql_db -o codeql_res.bqrs
codeql bqrs decode --format=json codeql.bqrs -o codeql.json 
```

#### Pass to Struct_Check
```
cargo run exe_path codeql.json
```

## TODO
    Make sure licensed as appropriate, given use of gimli example code
    Testing
        How to do CodeQL in an action, and then feed to this?