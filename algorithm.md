```tefcha
start

run: cli_parsing

if query_correct
    db_name = query.db_name
    if db_name.exist
        parsed_db =  DataModel{...} \n// GenericPatriciaMap ile. \n// 'DEL' için, olan keyword val'lere karsilik load etme!\n// maybe later, two differenet collections for different operations.
        run: rest of query with parsed_htable
        switch QUERY_TYPE
            case UPSERT
                
            case READ
                print: parsed_htable
            case DEL
                append: ROW with custom 'del' keyword val!
    else 
        eprint: db_not_exist_error
else 
    eprint: query_not_correct_error

end
```

 