*user_flow*:

$ start
    $ check_if_db_exist 
        ? {
            $ load_db
        } :
        {
            $ save_and_load_db
        }
    $ print status message

---

# DB Operations Examples