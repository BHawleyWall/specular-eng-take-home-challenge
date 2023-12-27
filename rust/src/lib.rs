pub mod merkle_tree {

    use crypto::sha2::Sha256;
    use crypto::digest::Digest;
    use std::vec::Vec;
    use std::result::Result;

    // Refactored common path to a helper function
    pub fn hasher(input: &str) -> String {
		let mut hasher = Sha256::new();
		hasher.input_str(input);
		hasher.result_str()
	}

    // hash function to be used for the construction of the merkle tree
    pub fn hash_leaf (leaf: &str) -> String {
        hasher(leaf)
    }

    // hash function to be used for the construction of the merkle tree
    pub fn hash_node(left: &str, right: &str) -> String {
        hasher(format!("{left}{right}").as_str())
    }

    #[derive(Clone)]
    pub struct MerkleNode {
        value: String,
		left: Option<Box<MerkleNode>>,
		right: Option<Box<MerkleNode>>
    }

	pub struct MerkleTree {
        leaves: Vec<String>,
        root_hash: String
	}

    pub struct MerkleProof {
        element:    String,         // element for which we want to prove inclusion
        siblings:   Vec<String>,    // path of siblings from the element up to the root
        directions: Vec<bool>       // signal if the sibling at the same index is on the left or right
    }

    // return the root hash of the merkle tree
	pub fn get_root(ref_tree: &MerkleTree) -> String {
		ref_tree.root_hash.to_owned()
	}

    // create a merkle tree from a list of elements
    // the tree should have the minimum height needed to contain all elements
    // empty slots should be filled with an empty string
    pub fn create_merkle_tree(elements: &Vec<String>) -> Result<MerkleTree, String> {
		let mut leaves = elements.to_owned();
		let mut nodes: Vec<MerkleNode> = Vec::new();

		leaf_pairwise_check(&mut leaves);

		for leaf in leaves.iter() {
			nodes.push(MerkleNode {
				value: hash_leaf(leaf),
				left: None,
				right: None
			});
		}

		while nodes.len() > 1 {
			nodes = generate_parent_row(nodes);
		}

		let root_hash = nodes[0].value.to_owned();

		Ok(MerkleTree {
			leaves,
			root_hash
		})
    }

    fn leaf_pairwise_check(leaves: &mut Vec<String>) {
        if leaves.len() % 2 == 1 {
			leaves.push(String::default());
		}
    }

    fn generate_parent_row(nodes: Vec<MerkleNode>) -> Vec<MerkleNode> {
		let mut parents: Vec<MerkleNode> = Vec::new();

		for i in (0..nodes.len()).step_by(2) {
			let left = nodes[i].value.to_owned();
			let right = nodes[i + 1].value.to_owned();

			parents.push(MerkleNode {
				value: hash_node(&left, &right),
				left: Some(Box::new(nodes[i].to_owned())),
				right: Some(Box::new(nodes[i + 1].to_owned()))
			});
		}

		parents
    }

    // return a merkle proof of the inclusion of element at the given index
    //
    // example:
    // proof for index 2 (marked with E), return the nodes marked `*` at each layer.
    //
    // tree:
    // d0:                                   [ R ]
    // d1:                [*]                                     [*]
    // d2:      [*]                 [*]                 [ ]                 [ ]
    // d3: [ ]       [ ]       [E]       [*]       [ ]       [ ]       [ ]       [ ]
    //
    // proof:
    // element    = E
    // siblings   = [d3-3, d2-0, d1-1]
    // directions = [false, true, false]
    pub fn get_proof(ref_tree: &MerkleTree, index: usize) -> Result<MerkleProof, String> {
        todo!()
    }

    // verify a merkle tree against a known root
    pub fn verify_proof(root: String, proof: &MerkleProof) -> bool {
        todo!()
    }

    // ** BONUS (optional - easy) **
    // Updates the Merkle tree (from leaf to root) to include the new element at index.
    // For simplicity, the index must be within the bounds of the original vector size.
    // If it is not, return an error.
    pub fn update_element(tree: MerkleTree, index: usize, element: &str) -> Result<MerkleTree, String> {
        todo!()
    }

    // ** BONUS (optional - hard) **
    // Generates a Merkle proof of the inclusion of contiguous elements,
    // starting at startIndex (inclusive) and ending at endIndex (exclusive).
    // If the indexes are out of bounds or startIndex >= endIndex, an error is returned.
    //
    // Note: modify the method signature to return your proof type.
    // Implement a separate verify_aggregate_proof for this type.
    //
    // The aggregated proof size should generally be smaller than
    // that of the naive approach (calling GetProof for every index).
    pub fn get_aggregate_proof(ref_tree: &MerkleTree, start_index: usize, end_index: usize) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::merkle_tree::*;

    #[test]
    fn test_root() {
        let elements = vec!["some".to_string(), "test".to_string(), "elements".to_string()];

        let expected_root = hash_node(
            &hash_node(
                &hash_leaf("some"),
                &hash_leaf("test")
                ),
            &hash_node(
                &hash_leaf("elements"),
                &hash_leaf("")
                )
            );

        let mt = create_merkle_tree(&elements);

        match mt {
            Ok(mt) => assert_eq!(get_root(&mt), expected_root),
            Err(e) => println!("{}", e)
        }
    }

    #[test]
    fn test_proof() {
        let elements = vec!["some".to_string(), "test".to_string(), "elements".to_string()];
        let mt = create_merkle_tree(&elements);

        match mt {
            Ok(mt) =>
                for i in 0..elements.len() {
                    let proof = get_proof(&mt, i);

                    match proof {
                        Ok(p) => assert!(verify_proof(get_root(&mt), &p)),
                        Err(e) => println!("{}", e)
                    }
                }
            Err(e) => println!("{}", e)
        }
    }
}
