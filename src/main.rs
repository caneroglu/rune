use std::fs::File;
use bincode::{config, decode_from_std_read, encode_into_std_write};
use patricia_tree::PatriciaMap;
use pest::Parser;
use rs_merkle::{Hasher, MerkleProof, MerkleTree};
 use sha2::{Digest, Sha256};
use uuid::Uuid;
use rune::datamodel::{DataModel, Sha256Algorithm};
use rune::parser::{RQLParser, Rule};
use rune::saveandload::{EncodeDecodeDataMethods, EncodeDecodeDataModel};
/*
TODO: Şöyle bir tablette dizayn et bakalım, UX açısından nasıl daha iyi hale getirirsin Patricia tree için?
- Mesela, 2025 -> "a" gibi *encoding* yapabilirsin, bu sayede "a" önek araması yapıldığında hızlıca 2025 için sorgular gelir. *encoding* O(1) olduğundan zamansal olarak yük yok.
- İkincil Adres defteri DB kullanmak lazım. Eğer NoSQL gibi bir şey olmasını istiyorsam. Simple K/V için geçersiz... *sanırım istiyorum.* Çünkü günlük hayatta kullanabilirim.
- Ya da belki, iki farklı sürüm yazarım. Birincisi sadece simple K/V diğeri ise döküman tutan gelişmiş versiyonu.


TODO: Adres defteri ile UUID saklarım bir *key* için, sorgu aşamasında UUID isterim, böylelikle iki "Ali" arasında farklılık olur. Adres defteri indeksini de patricia tree ile yazabilirim. *get* metodu *full match* istiyor zaten.
TODO: Eğer performans kaybı olursa, bu indeks'i farklı kütüphane ile yazabilirim. Şu anlık gerek yok. Sonra bakarız.

TODO: Saklanan veri tipini gösteren *önek*te yaratmalı mıyım? yoksa şu aşamada fazla kompleks mi olur? Yaratalım. örn, 'b' = binary, 'j' = jpeg gibi bir şey olsun. Sınırlı olması yeterli. Popüler olursa geliştiririz. Şu aşamada gereksiz.

TODO!: PARSER DİLİNİ SİKTİR ET. SİMPLE K/V. BASİT DİL YAZABİLİRSİN BUNA UYGUN!

*/

fn main() {
    let v1 = DataModel::new(
        "main".to_string(),
        chrono::Utc::now(),
        Uuid::new_v4(),
        "dene-key".to_string(),
        "deneme_veri1".to_owned()
    );

    let v2 = DataModel::new(
        "main".to_string(),
        chrono::Utc::now(),
        Uuid::new_v4(),
        "dene-key2".to_string(),
        "deneme_veri2".to_owned()
    );

    let ornek_query = "+=db.key1 = key2";

    RQLParser::parse_query(ornek_query.to_owned());

    let a1  = EncodeDecodeDataModel{data: v1.clone()};
    let a2  = EncodeDecodeDataModel{data: v2.clone()};

    let a3 = vec![a1,a2];

    //a3.encode_and_save("sil.bin".to_string());


    let mut map = PatriciaMap::new();

    let a4 = Vec::<EncodeDecodeDataModel>::load_and_decode("sil.bin".to_string()).unwrap();

    a4.clone().into_iter().for_each(|x| {
        map.insert(x.data.column.clone(), x.data);
    });

    map.iter().for_each(|(k,v)| {
        println!("key_patricia: {}\n, val_patricia: {}", String::from_utf8(k).unwrap(), &v);
    });


    let markle_leafs: Vec<[u8;32]> = map.iter().map(|patricia_elem| {
        Sha256Algorithm::hash(format!("{}{}",String::from_utf8(patricia_elem.0).unwrap(),patricia_elem.1).as_bytes())
    }).collect();

    let markle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&markle_leafs);

    markle_leafs.iter().for_each(|hashed_patricia_elem| {
        println!("{}",hex::encode(hashed_patricia_elem));
    });

    let kontrol_edilecek_indisler = vec![0,1];
    let markle_kontrol_edilecek_yapraklar = markle_leafs.get(0..=1).unwrap();

    let merkle_proof = markle_tree.proof(&kontrol_edilecek_indisler);
    let merkle_kok = markle_tree.root().unwrap();

    let proof_bytes = merkle_proof.to_bytes();

    let proof = MerkleProof::<Sha256Algorithm>::try_from(proof_bytes.as_slice()).unwrap();

    if proof.verify(merkle_kok,&kontrol_edilecek_indisler,markle_kontrol_edilecek_yapraklar,markle_leafs.len()) {
        println!("PROOF OK!")
    }else { println!("PROOF ERROR!") }






    // let ds: Vec<(Vec<u8>, &String)> = map.iter_prefix(b"main.202507").collect();

/*    for row in ds.into_iter() {
        println!("Key: {}\n Val: {}",String::from_utf8(row.0).unwrap(),row.1)
    }*/

}

fn process_pair(pair: pest::iterators::Pair<Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!("{}{:?}: {}", indent_str, pair.as_rule(), pair.as_str());

    for inner_pair in pair.into_inner() {
        process_pair(inner_pair, indent + 1);
    }
}