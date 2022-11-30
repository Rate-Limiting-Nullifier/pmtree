use pmtree::*;

use rln::circuit::Fr;
use rln::circuit::TEST_TREE_HEIGHT;
use rln::poseidon_hash::poseidon_hash;
use rln::protocol::hash_to_field;
use rln::utils::str_to_fr;

use std::collections::HashMap;
use std::io::Cursor;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

struct MemoryDB(HashMap<DBKey, Value>);

#[derive(Clone, Copy, PartialEq, Eq)]
struct PoseidonHash;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct MyFr(Fr);

impl Hasher for PoseidonHash {
    type Fr = MyFr;

    fn default_leaf() -> Self::Fr {
        MyFr(Fr::from(0))
    }

    fn hash(inputs: &[Self::Fr]) -> Self::Fr {
        MyFr(poseidon_hash(
            &inputs.iter().map(|&x| x.0).collect::<Vec<_>>(),
        ))
    }
}

impl From<Vec<u8>> for MyFr {
    fn from(v: Vec<u8>) -> Self {
        let c = Cursor::new(v);
        let f = Fr::deserialize(c).unwrap();
        MyFr(f)
    }
}

impl From<MyFr> for Vec<u8> {
    fn from(v: MyFr) -> Self {
        let mut buf = vec![];

        Fr::serialize(&v.0, &mut buf).unwrap();

        buf
    }
}

impl Database for MemoryDB {
    fn new(_dbpath: &str) -> Result<Self> {
        Ok(MemoryDB(HashMap::new()))
    }

    fn load(_dbpath: &str) -> Result<Self> {
        Err(Error("Cannot load in-memory DB".to_string()))
    }

    fn get(&self, key: DBKey) -> Result<Option<Value>> {
        Ok(self.0.get(&key).cloned())
    }

    fn put(&mut self, key: DBKey, value: Value) -> Result<()> {
        self.0.insert(key, value);

        Ok(())
    }
}

#[test]
fn poseidon_memory() -> Result<()> {
    let mut mt = MerkleTree::<MemoryDB, PoseidonHash>::new(TEST_TREE_HEIGHT, "abacaba")?;

    let leaf_index = 3;

    let identity_secret = hash_to_field(b"test-merkle-proof");
    let id_commitment = MyFr(poseidon_hash(&[identity_secret]));

    // let default_leaf = Fr::from(0);
    mt.set(leaf_index, id_commitment).unwrap();

    // We check correct computation of the root
    let root = mt.root();

    assert_eq!(
        root.0,
        str_to_fr(
            "0x21947ffd0bce0c385f876e7c97d6a42eec5b1fe935aab2f01c1f8a8cbcc356d2",
            16
        )
    );

    let merkle_proof = mt.proof(leaf_index).expect("proof should exist");
    let path_elements = merkle_proof.get_path_elements();
    let identity_path_index = merkle_proof.get_path_index();

    // We check correct computation of the path and indexes
    // These values refers to TEST_TREE_HEIGHT == 16
    let expected_path_elements = vec![
        MyFr(str_to_fr(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            16,
        )),
        MyFr(str_to_fr(
            "0x2098f5fb9e239eab3ceac3f27b81e481dc3124d55ffed523a839ee8446b64864",
            16,
        )),
        MyFr(str_to_fr(
            "0x1069673dcdb12263df301a6ff584a7ec261a44cb9dc68df067a4774460b1f1e1",
            16,
        )),
        MyFr(str_to_fr(
            "0x18f43331537ee2af2e3d758d50f72106467c6eea50371dd528d57eb2b856d238",
            16,
        )),
        MyFr(str_to_fr(
            "0x07f9d837cb17b0d36320ffe93ba52345f1b728571a568265caac97559dbc952a",
            16,
        )),
        MyFr(str_to_fr(
            "0x2b94cf5e8746b3f5c9631f4c5df32907a699c58c94b2ad4d7b5cec1639183f55",
            16,
        )),
        MyFr(str_to_fr(
            "0x2dee93c5a666459646ea7d22cca9e1bcfed71e6951b953611d11dda32ea09d78",
            16,
        )),
        MyFr(str_to_fr(
            "0x078295e5a22b84e982cf601eb639597b8b0515a88cb5ac7fa8a4aabe3c87349d",
            16,
        )),
        MyFr(str_to_fr(
            "0x2fa5e5f18f6027a6501bec864564472a616b2e274a41211a444cbe3a99f3cc61",
            16,
        )),
        MyFr(str_to_fr(
            "0x0e884376d0d8fd21ecb780389e941f66e45e7acce3e228ab3e2156a614fcd747",
            16,
        )),
        MyFr(str_to_fr(
            "0x1b7201da72494f1e28717ad1a52eb469f95892f957713533de6175e5da190af2",
            16,
        )),
        MyFr(str_to_fr(
            "0x1f8d8822725e36385200c0b201249819a6e6e1e4650808b5bebc6bface7d7636",
            16,
        )),
        MyFr(str_to_fr(
            "0x2c5d82f66c914bafb9701589ba8cfcfb6162b0a12acf88a8d0879a0471b5f85a",
            16,
        )),
        MyFr(str_to_fr(
            "0x14c54148a0940bb820957f5adf3fa1134ef5c4aaa113f4646458f270e0bfbfd0",
            16,
        )),
        MyFr(str_to_fr(
            "0x190d33b12f986f961e10c0ee44d8b9af11be25588cad89d416118e4bf4ebe80c",
            16,
        )),
        MyFr(str_to_fr(
            "0x22f98aa9ce704152ac17354914ad73ed1167ae6596af510aa5b3649325e06c92",
            16,
        )),
        MyFr(str_to_fr(
            "0x2a7c7c9b6ce5880b9f6f228d72bf6a575a526f29c66ecceef8b753d38bba7323",
            16,
        )),
        MyFr(str_to_fr(
            "0x2e8186e558698ec1c67af9c14d463ffc470043c9c2988b954d75dd643f36b992",
            16,
        )),
        MyFr(str_to_fr(
            "0x0f57c5571e9a4eab49e2c8cf050dae948aef6ead647392273546249d1c1ff10f",
            16,
        )),
        MyFr(str_to_fr(
            "0x1830ee67b5fb554ad5f63d4388800e1cfe78e310697d46e43c9ce36134f72cca",
            16,
        )),
    ];

    let expected_identity_path_index: Vec<u8> =
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    assert_eq!(path_elements, expected_path_elements);
    assert_eq!(identity_path_index, expected_identity_path_index);

    // We check correct verification of the proof
    assert!(mt.verify(&id_commitment, &merkle_proof));

    Ok(())
}
