use rune::RQLParser;

//FIXME: Fix pipeline and comment and USE proper testing framework!

#[test]
fn upsert_with_flags_and_json_value() {
    RQLParser::parse_query(r#"+ users:alice={"name":"Can","age":27} [ttl=60,nx]"#)
        .expect("parse ok");
}

#[test]
fn read_exact_and_radix() {
    RQLParser::parse_query(r#"@ users:alice"#).expect("parse ok"); // exact (:)
    RQLParser::parse_query(r#"@ users.al"#).expect("parse ok"); // radix (.)
}

#[test]
fn delete_exact_and_prefix() {
    RQLParser::parse_query(r#"- users:deadbeef"#).expect("parse ok");
    RQLParser::parse_query(r#"- users.al"#).expect("parse ok");
}

#[test]
fn rename_with_nx_flag() {
    RQLParser::parse_query(r#"+= users:old=users:new [nx]"#).expect("parse ok");
}

#[test]
fn quoted_key_and_value_with_specials() {
    // boşluklu key + içinde ; ve > barındıran value (quoted)
    RQLParser::parse_query(r#"+ users:"weird key"="weird ; value > still inside quotes""#)
        .expect("parse ok");
}

#[test]
fn pipeline_basic() {
    // '>' value RAW tarafından yutulmaz; pipeline oluşmalı
    RQLParser::parse_query(r#"+ users:alice=foo > @ users:alice"#).expect("parse ok");
}

#[test]
fn pipeline_with_spaces_and_newlines() {
    RQLParser::parse_query(
        r#"
        + users:alice=foo
        >
        @ users:alice
        "#,
    )
    .expect("parse ok");
}

#[test]
fn quoted_gt_does_not_break_pipeline() {
    // quoted içindeki '>' value'nun parçası; pipeline değil
    RQLParser::parse_query(r#"+ users:msg="hello > world""#).expect("parse ok");
    // ama sonrasında gerçek pipeline gelebilir
    RQLParser::parse_query(r#"+ users:msg="hello > world" > - users:msg"#).expect("parse ok");
}

#[test]
fn comments_and_semicolons() {
    RQLParser::parse_query(
        r#"
        # ilk satır yorum
        + users:alice=1 [xx];  # inline yorum
        @ users:alice;
        - users.al
        "#,
    )
    .expect("parse ok");
}

#[test]
fn flags_validation() {
    // Geçerli
    RQLParser::parse_query(r#"+ users:exp=42 [ttl=300]"#).expect("parse ok");
    RQLParser::parse_query(r#"+ users:exp=42 [nx]"#).expect("parse ok");
    RQLParser::parse_query(r#"+ users:exp=42 [xx]"#).expect("parse ok");
    RQLParser::parse_query(r#"+ users:exp=42 [ttl=1,nx]"#).expect("parse ok");

    /*     // Geçersiz flag adı veya ttl değeri -> parse hatası
    parse_err(r#"+ users:exp=42 [foo]"#).expect("parse ok");
    parse_err(r#"+ users:exp=42 [ttl=x]"#).expect("parse ok"); */
}

#[test]
fn obvious_errors() {
    /*     // Eksik value
    parse_err(r#"+ users:alice="#).expect("parse ok");

    // Yarım pipeline
    parse_err(r#"+ users:alice=foo > "#).expect("parse ok");

    // Tek başına '>' olamaz
    parse_err(r#"@ users:al>"#).expect("parse ok");

    // Rename'de ikinci key eksik
    parse_err(r#"+= users:old="#).expect("parse ok");

    // Kapanmayan string
    parse_err(r#"+ users:alice="unterminated"#).expect("parse ok"); */
}

#[test]
fn upsert_without_flags() {
    let q = r#"+ dene:ali=veli"#;
    let cmd = RQLParser::parse_query(q).expect("parse ok");
    // Komut::Upsert {... flags: None } beklenir
}

#[test]
fn upsert_with_nx_flag() {
    let q = r#"+ dene:ali=veli [nx]"#;
    let cmd = RQLParser::parse_query(q).expect("parse ok");
}

#[test]
fn upsert_with_ttl_and_xx() {
    let q = r#"+ dene:ali=veli [ttl=60,xx]"#;
    let cmd = RQLParser::parse_query(q).expect("parse ok");
}

#[test]
fn read_and_delete() {
    RQLParser::parse_query(r#"@ dene:ali"#).expect("ok");
    RQLParser::parse_query(r#"- dene.al"#).expect("ok");
}

#[test]
fn rename_with_optional_flag_ignored_for_now() {
    RQLParser::parse_query(r#"+= dene:old=dene:new [nx]"#).expect("ok");
}

#[test]
fn pipeline_doesnt_get_eaten_by_value() {
    let q = r#"+ dene:ali=foo > @ dene:ali"#;
    RQLParser::parse_query(q).expect("ok");
}
