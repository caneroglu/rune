```tefcha
parse_query: {\n pub db_name: String,\n db_path: PathBuf, \n pub key: String,\n pub access_mode: Exact|Radix,\n pub val: String,\n pub flags: Flags\n}

if ExactAccess
    if flags == NX || flags.is_empty()\n// default: create and append
        if !file.is_exist()
            create.file()\nappend.file()
        else
            open.file()\nappend.file()
    else
        if !file.is_exist()
            Error!
        else
            open.file()
            key.previous_search()
            if key.exist()
                calculate.prev_off()\nappend.file()
            else
                Error!
elif RadixAccess
    if !file.is_exist()
        Error!
    else
        open.file()
        parse_query into GenericPatriciaTree
        // IMPLEMENT LIMIT
else
    Error!
    
```

