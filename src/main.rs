
use patricia_tree::PatriciaMap;
use rs_merkle::{Hasher, MerkleProof, MerkleTree};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use rune::datamodel::{DataModel, Sha256Algorithm};

/*
TODO: Şöyle bir tablette dizayn et bakalım, UX açısından nasıl daha iyi hale getirirsin Patricia tree için?
- Mesela, 2025 -> "a" gibi *encoding* yapabilirsin, bu sayede "a" önek araması yapıldığında hızlıca 2025 için sorgular gelir. *encoding* O(1) olduğundan zamansal olarak yük yok.
- İkincil Adres defteri DB kullanmak lazım. Eğer NoSQL gibi bir şey olmasını istiyorsam. Simple K/V için geçersiz... *sanırım istiyorum.* Çünkü günlük hayatta kullanabilirim.
- Ya da belki, iki farklı sürüm yazarım. Birincisi sadece simple K/V diğeri ise döküman tutan gelişmiş versiyonu.


TODO: Adres defteri ile UUID saklarım bir *key* için, sorgu aşamasında UUID isterim, böylelikle iki "Ali" arasında farklılık olur. Adres defteri indeksini de patricia tree ile yazabilirim. *get* metodu *full match* istiyor zaten.
TODO: Eğer performans kaybı olursa, bu indeks'i farklı kütüphane ile yazabilirim. Şu anlık gerek yok. Sonra bakarız.

TODO: Saklanan veri tipini gösteren *önek*te yaratmalı mıyım? yoksa şu aşamada fazla kompleks mi olur? Yaratalım. örn, 'b' = binary, 'j' = jpeg gibi bir şey olsun. Sınırlı olması yeterli. Popüler olursa geliştiririz. Şu aşamada gereksiz.


*/

fn main() {
    let v1 = DataModel::new(
        "main".to_string(),
        chrono::Utc::now(),
        Uuid::new_v4(),
        "dene-key".to_string(),
    );

    let v2 = DataModel::new(
        "main".to_string(),
        chrono::Utc::now(),
        Uuid::new_v4(),
        "dene-key2".to_string(),
    );

    println!("{}", v1.to_string());

    let mut map = PatriciaMap::new();


    let komple_vc = vec![(v1.to_string(), "deneme veri1".to_owned()),(v2.to_string(), "deneme veri2".to_owned())];
    let yapraklar: Vec<[u8;32]>  = komple_vc.iter().map(|(k,v)| Sha256Algorithm::hash(format!("{}{}",k,v).as_bytes())).collect();

    yapraklar.iter().for_each(|h| {
        println!("{}", hex::encode(h));
    });

    let m_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&yapraklar);

    let kontrol_edilecek_indeksler = vec![0,1];
    let leaves_t_p = yapraklar.get(0..1).unwrap();

    let merkle_proof = m_tree.proof(&kontrol_edilecek_indeksler);
    let merkle_root = m_tree.root().unwrap();

    let proof_bytes = merkle_proof.to_bytes();

    let proof = MerkleProof::<Sha256Algorithm>::try_from(proof_bytes.as_slice()).unwrap();

    if proof.verify(merkle_root,&kontrol_edilecek_indeksler,leaves_t_p,yapraklar.len()) {
        println!("PROOF OK!")
    }else { println!("PROOF ERROR!") } // Error attı. Sanırım Utc now'dan ötürü. Yarın, *bincode* entegrasyonu ile tekrar denerim -  TODO!


    map.insert(v1.to_string(),"Deneme veri1".to_string());
    map.insert(v2.to_string(),"Deneme veri2".to_string());

    let ds: Vec<(Vec<u8>, &String)> = map.iter_prefix(b"main.202507").collect();

/*    for row in ds.into_iter() {
        println!("Key: {}\n Val: {}",String::from_utf8(row.0).unwrap(),row.1)
    }*/

}