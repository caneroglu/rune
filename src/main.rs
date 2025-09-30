use std::fs::File;
use bincode::{config, decode_from_std_read, encode_into_std_write};
use clap::Parser;
use patricia_tree::PatriciaMap;
 
use rs_merkle::{Hasher, MerkleProof, MerkleTree};
 
use rune::cli::interface::Command;
use sha2::{Digest, Sha256};
use uuid::Uuid;
 
use rune::query::{RQLParser, CommandExecutor};
use rune::query::parser::Rule;
 
 

fn main() -> Result<(), anyhow::Error> {
     Command::parse_command()
}

 

/*

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

    let ornek_query = "-db.dene*";

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
*/