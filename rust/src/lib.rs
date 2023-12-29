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

    #[derive(Clone, Debug)]
    pub struct MerkleNode {
        value: String,
		left: Option<Box<MerkleNode>>,
		right: Option<Box<MerkleNode>>
    }

    impl From<String> for MerkleNode {
		fn from(value: String) -> Self {
			MerkleNode {
				value: hash_leaf(&value),
				left: None,
				right: None
			}
		}
	}

	#[derive(Debug)]
	pub struct MerkleTree {
        leaves: Vec<String>,
        pub(crate) root_hash: String
	}

    #[derive(Debug)]
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

		leaf_pairwise_check(&mut leaves);

		let mut nodes: Vec<MerkleNode> = leaves.iter().map(|e| e.to_owned().into()).collect::<_>();

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

    fn generate_parent(left: MerkleNode, right: MerkleNode) -> MerkleNode {
        MerkleNode {
			value: hash_node(&left.value, &right.value),
			left: Some(Box::new(left)),
			right: Some(Box::new(right))
		}
    }

    fn generate_parent_row(nodes: Vec<MerkleNode>) -> Vec<MerkleNode> {
		let mut parents: Vec<MerkleNode> = Vec::new();

		for i in (0..nodes.len()).step_by(2) {
			parents.push(generate_parent(nodes[i].to_owned(), nodes[i + 1].to_owned()));
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
		if index >= ref_tree.leaves.len() {
			return Err("Index of the target element is out of bounds for this tree".to_string());
		}

		let element = ref_tree.leaves[index].to_owned();
		let mut siblings: Vec<String> = Vec::new();
		let mut directions: Vec<bool> = Vec::new();

        let mut current_row: Vec<MerkleNode> = ref_tree.leaves
                .to_owned()
                .iter()
                .map(|leaf| leaf.to_owned().into())
                .collect::<_>();
		let mut current_node = current_row[index].to_owned();

		while current_row.len() > 1 {
			let current_index = current_row
					.iter()
					.position(|node| node.value.eq(&current_node.value))
					.expect(
						format!(
							"Should have been able to locate the generated node ({current_node:#?}) in the row ({current_row:#?})\
                             Check the node and row generator paths to verify."
						).as_str()
                    );
			let sibling_is_left_child = !current_index % 2 == 0;

            if sibling_is_left_child {
				siblings.push(current_row[current_index - 1].value.to_owned());
			} else {
				siblings.push(current_row[current_index + 1].value.to_owned());
			}

			directions.push(sibling_is_left_child);

            current_row = generate_parent_row(current_row);
			current_node = current_row[current_index / 2].to_owned();
		}

		Ok(MerkleProof {
			element,
			siblings,
			directions
        })
    }

    // verify a merkle sub-tree against a known root
    pub fn verify_proof(root: String, proof: &MerkleProof) -> bool {
		let mut current_hash = hash_leaf(&proof.element);
        
		proof.siblings.iter().zip(proof.directions.iter()).for_each(|(sibling, is_left_child)| {
			current_hash = if *is_left_child {
				hash_node(&sibling, &current_hash)
			} else {
				hash_node(&current_hash, &sibling)
			};
        });

		current_hash.eq(&root)
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
mod validations {
    use crate::merkle_tree::*;
    

    const TEST_ELEMENTS: [&str; 3] = ["some", "test", "elements"];

    fn get_test_tree() -> MerkleTree {
		let elements = TEST_ELEMENTS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
		create_merkle_tree(&elements).expect("Should have received a valid tree given const test inputs")
	}

    #[test]
    fn getting_root_hashes() {
		let mt = get_test_tree();
        let expected_root_hash = mt.root_hash.to_owned();

		assert_eq!(get_root(&mt), expected_root_hash);
	}

    #[test]
    fn verifying_proofs() {
		let mt = get_test_tree();

        let proof = get_proof(&mt, 0).expect("Should have received a valid proof for the first element");

		assert!(verify_proof(get_root(&mt), &proof));
        assert_eq!(verify_proof("not_a_valid_hash".into(), &proof), false);
	}

    #[test]
    fn test_root() {
        let expected_root = hash_node(
            &hash_node(
                &hash_leaf(TEST_ELEMENTS[0]),
                &hash_leaf(TEST_ELEMENTS[1])
                ),
            &hash_node(
                &hash_leaf(TEST_ELEMENTS[2]),
                &hash_leaf("")
                )
            );

        let mt = get_test_tree();
		
        assert_eq!(get_root(&mt), expected_root);
    }

    #[test]
    fn test_proof() {
        let mt = get_test_tree();

		for i in 0..TEST_ELEMENTS.len() {
			let proof = get_proof(&mt, i).expect("Should have received a valid proof for any of the original elements");

			assert!(verify_proof(get_root(&mt), &proof))
		}
    }
}
