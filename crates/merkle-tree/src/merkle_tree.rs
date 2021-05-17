use alloc::vec::Vec;
use core::mem::size_of;

use once_cell::sync::Lazy;
use tiny_keccak::{Hasher, Keccak};
use zkp_u256::{Zero, U256};

use super::*;

// ceil(log2(1<<20))
static TREE_DEPTH: usize = 10;

// 1<<20 leaves
const MAX_LEAF_COUNT: usize = 1024;

static FILL_LEVEL_IVS: Lazy<Vec<U256>> = Lazy::new(|| {
    let ivs = [
        "149674538925118052205057075966660054952481571156186698930522557832224430770",
        "9670701465464311903249220692483401938888498641874948577387207195814981706974",
        "18318710344500308168304415114839554107298291987930233567781901093928276468271",
        "6597209388525824933845812104623007130464197923269180086306970975123437805179",
        "21720956803147356712695575768577036859892220417043839172295094119877855004262",
        "10330261616520855230513677034606076056972336573153777401182178891807369896722",
        "17466547730316258748333298168566143799241073466140136663575045164199607937939",
        "18881017304615283094648494495339883533502299318365959655029893746755475886610",
        "21580915712563378725413940003372103925756594604076607277692074507345076595494",
        "12316305934357579015754723412431647910012873427291630993042374701002287130550",
        "18905410889238873726515380969411495891004493295170115920825550288019118582494",
        "12819107342879320352602391015489840916114959026915005817918724958237245903353",
        "8245796392944118634696709403074300923517437202166861682117022548371601758802",
        "16953062784314687781686527153155644849196472783922227794465158787843281909585",
        "19346880451250915556764413197424554385509847473349107460608536657852472800734",
        "14486794857958402714787584825989957493343996287314210390323617462452254101347",
        "11127491343750635061768291849689189917973916562037173191089384809465548650641",
        "12217916643258751952878742936579902345100885664187835381214622522318889050675",
        "722025110834410790007814375535296040832778338853544117497481480537806506496",
        "15115624438829798766134408951193645901537753720219896384705782209102859383951",
        "11495230981884427516908372448237146604382590904456048258839160861769955046544",
        "16867999085723044773810250829569850875786210932876177117428755424200948460050",
        "1884116508014449609846749684134533293456072152192763829918284704109129550542",
        "14643335163846663204197941112945447472862168442334003800621296569318670799451",
        "1933387276732345916104540506251808516402995586485132246682941535467305930334",
        "7286414555941977227951257572976885370489143210539802284740420664558593616067",
        "16932161189449419608528042274282099409408565503929504242784173714823499212410",
        "16562533130736679030886586765487416082772837813468081467237161865787494093536",
        "6037428193077828806710267464232314380014232668931818917272972397574634037180",
    ];
    ivs.iter()
        .map(|iv| U256::from_decimal_str(iv).unwrap())
        .collect::<Vec<_>>()
});

#[derive(Clone, Debug)]
struct MerkleTree {
    cur: usize,
    root: U256,
    leaves: Vec<Vec<U256>>,
}

impl Default for MerkleTree {
    fn default() -> Self {
        let leaves = (0..TREE_DEPTH + 1)
            .rev()
            .map(|x| vec![U256::zero(); 2usize.pow(x as u32)])
            .collect::<Vec<_>>();

        let mut mt = MerkleTree {
            cur: 0,
            root: U256::zero(),
            leaves,
        };
        mt.init();
        mt
    }
}

impl MerkleTree {
    fn init(&mut self) {
        for depth in 0..TREE_DEPTH {
            self.leaves[depth]
                .clone()
                .chunks_exact(2)
                .enumerate()
                .for_each(|(index, chunk)| {
                    self.leaves[depth][index * 2] =
                        Self::get_unique_leaf(depth, index * 2, chunk[0].clone());
                    self.leaves[depth][index * 2 + 1] =
                        Self::get_unique_leaf(depth, index * 2 + 1, chunk[1].clone());
                    self.leaves[depth + 1][index] = Self::hash_impl(
                        &self.leaves[depth][index * 2],
                        &self.leaves[depth][index * 2 + 1],
                        &FILL_LEVEL_IVS[depth],
                    );
                })
        }
    }

    fn insert(&mut self, message: &[u8]) -> Result<(U256, usize), &'static str> {
        let leaf = mimc(message);
        if leaf.is_zero() {
            return Err("leaf must be non-zero");
        }

        let offset = self.cur;
        self.leaves[0][self.cur] = leaf.clone();

        self.root = self.update();
        self.cur += 1;

        Ok((leaf, offset))
    }

    // Update the leaves of the entire tree, return new tree root.
    fn update(&mut self) -> U256 {
        let mut current_index = self.cur;
        let mut leaf1: U256;
        let mut leaf2: U256;

        for depth in 0..TREE_DEPTH {
            let next_index = current_index / 2;
            if current_index % 2 == 0 {
                leaf1 = self.leaves[depth][current_index].clone();
                leaf2 = MerkleTree::get_unique_leaf(
                    depth,
                    current_index + 1,
                    self.leaves[depth][current_index + 1].clone(),
                );
            } else {
                leaf1 = MerkleTree::get_unique_leaf(
                    depth,
                    current_index - 1,
                    self.leaves[depth][current_index - 1].clone(),
                );
                leaf2 = self.leaves[depth][current_index].clone();
            }
            self.leaves[depth + 1][next_index] =
                MerkleTree::hash_impl(&leaf1, &leaf2, &FILL_LEVEL_IVS[depth]);
            current_index = next_index;
        }
        self.root = self.leaves[TREE_DEPTH][0].clone();
        self.root.clone()
    }

    // Return leaf according to depth and index,
    fn get_leaf(&self, depth: usize, offset: usize) -> U256 {
        MerkleTree::get_unique_leaf(depth, offset, self.leaves[depth][offset].clone())
    }

    // get merkle tree root
    fn get_root(&self) -> U256 {
        self.root.clone()
    }

    // Obtain the merkel proof according to the corresponding leaf of the index
    fn get_proof(&self, mut index: usize) -> Vec<U256> {
        let mut address_bits = vec![false; TREE_DEPTH];
        let mut proof_path = vec![U256::zero(); TREE_DEPTH];

        for depth in 0..TREE_DEPTH {
            address_bits[depth] = index % 2 == 0;
            if index % 2 == 0 {
                proof_path[depth] = self.get_leaf(depth, index + 1);
            } else {
                proof_path[depth] = self.get_leaf(depth, index - 1);
            }
            index /= 2;
        }
        proof_path
    }

    //
    fn verify_merkle_proof(&self, leaf: U256, proof: Vec<U256>, index: usize) -> bool {
        if proof.len() != TREE_DEPTH && index > MAX_LEAF_COUNT {
            return false;
        }
        self.verify_path(leaf, proof, index) == self.get_root()
    }

    // Returns calculated merkle root
    fn verify_path(&self, leaf: U256, in_path: Vec<U256>, mut index: usize) -> U256 {
        let mut item = leaf;
        for depth in 0..TREE_DEPTH {
            if index % 2 == 0 {
                item = MerkleTree::hash_impl(&item, &in_path[depth], &FILL_LEVEL_IVS[depth]);
            } else {
                item = MerkleTree::hash_impl(&in_path[depth], &item, &FILL_LEVEL_IVS[depth]);
            }
            index /= 2;
        }
        item
    }

    //
    fn get_unique_leaf(depth: usize, offset: usize, mut leaf: U256) -> U256 {
        if leaf.is_zero() {
            // Keccak(depth, offset)
            let mut input = [0u8; 32];
            input[0..size_of::<usize>()].copy_from_slice(&depth.to_be_bytes()[..]);
            input[size_of::<usize>()..2 * size_of::<usize>()]
                .copy_from_slice(&offset.to_be_bytes()[..]);

            let mut keccak = Keccak::v256();
            let mut received = [0u8; 32];
            keccak.update(&input[..]);
            keccak.finalize(&mut received);

            leaf = U256::from_bytes_be(&received);
        }
        leaf
    }

    // Use two leaves to generate mimc hash
    fn hash_impl(left: &U256, right: &U256, iv: &U256) -> U256 {
        let input = vec![left, right];
        mimc_with_key(input, iv)
    }
}

#[test]
fn test_merkle_tree() {
    let mut mt = MerkleTree::default();
    let message = b"hello world";
    let (leaf, index) = mt.insert(message).unwrap();
    assert_eq!(mt.update(), mt.get_root());

    let merkle_proof = mt.get_proof(index);
    assert!(mt.verify_merkle_proof(leaf, merkle_proof, index));
}
