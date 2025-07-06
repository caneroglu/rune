use clap::Parser;


/*

Maybe after i can implement custom language for this DB like SQL. Using Pest crate. We do not need right now.
NOPE, for utilizing Patricia Tree - i need to implement *simple* query language.

1. rune help

2. rune create table _tablo adı_
3. rune delete rable _tablo adı_
4. rune update table name _tabloadı_ _yeni tablo adı_
5. rune read table _tablename_
5. rune create user _user nname_
6. rune delete user _user name_
7. rune read user _username_
8. rune update user name _eski username_ yeni username_


rune create data _tablo adı_ _user id_ _column name_ {veri}
rune delete data _
*/

#[derive(Parser, Debug)]
struct Argumanlar {}