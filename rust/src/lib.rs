pub mod merkle_tree {

    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use std::result::Result;
    use std::vec::Vec;

    // Refactored common path to a helper function
    pub fn hasher(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(input);
        hasher.result_str()
    }

    // hash function to be used for the construction of the merkle tree
    pub fn hash_leaf(leaf: &str) -> String {
        hasher(leaf)
    }

    // hash function to be used for the construction of the merkle tree
    pub fn hash_node(left: &str, right: &str) -> String {
        hasher(format!("{left}{right}").as_str())
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug, Default)]
    pub struct MerkleNode {
        value: String,
        left: Option<Box<MerkleNode>>,
        right: Option<Box<MerkleNode>>,
    }

    impl From<String> for MerkleNode {
        fn from(value: String) -> Self {
            MerkleNode {
                value: hash_leaf(&value),
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug)]
    pub struct MerkleTree {
        pub(crate) leaves: Vec<String>,
        pub(crate) root_hash: String,
    }

    #[derive(Debug)]
    pub struct MerkleProof {
        element: String,       // element for which we want to prove inclusion
        siblings: Vec<String>, // path of siblings from the element up to the root
        directions: Vec<bool>, // signal if the sibling at the same index is on the left or right
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct MerkleAggregateProof {
        elements: Vec<String>, // range of elements for which we want to prove inclusion, in left-to-right order as present in the tree
        siblings: Vec<String>, // path of siblings from the elements up to the last level of siblings necessary to generate the remainder up to the root
        directions: Vec<bool>, // signal if the siblings at the same depth are on the left
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

        Ok(MerkleTree { leaves, root_hash })
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
            right: Some(Box::new(right)),
        }
    }

    fn generate_parent_row(nodes: Vec<MerkleNode>) -> Vec<MerkleNode> {
        let mut parents: Vec<MerkleNode> = Vec::new();

        nodes
            .chunks_exact(2)
            .for_each(|pair| parents.push(generate_parent(pair[0].to_owned(), pair[1].to_owned())));

        nodes
            .chunks_exact(2)
            .remainder()
            .iter()
            .for_each(|node| parents.push(generate_parent(node.to_owned(), MerkleNode::default())));

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

        let mut current_row: Vec<MerkleNode> = ref_tree
            .leaves
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
							"Should have been able to locate the generated node in the row\
                             Check the node and row generators at the bottom of the loop to verify."
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
            directions,
        })
    }

    // verify a merkle sub-tree against a known root
    pub fn verify_proof(root: String, proof: &MerkleProof) -> bool {
        let mut current_hash = hash_leaf(&proof.element);

        proof
            .siblings
            .iter()
            .zip(proof.directions.iter())
            .for_each(|(sibling, is_left_child)| {
                current_hash = if *is_left_child {
                    hash_node(sibling, &current_hash)
                } else {
                    hash_node(&current_hash, sibling)
                };
            });

        current_hash.eq(&root)
    }

    // ** BONUS (optional - easy) **
    // Updates the Merkle tree (from leaf to root) to include the new element at index.
    // For simplicity, the index must be within the bounds of the original vector size.
    // If it is not, return an error.
    pub fn update_element(
        tree: MerkleTree,
        index: usize,
        element: &str,
    ) -> Result<MerkleTree, String> {
        if index >= tree.leaves.len() {
            return Err("Index of the target element is out of bounds for this tree".to_string());
        }

        let mut elements = tree.leaves;
        elements.retain(|e| !e.is_empty());
        elements.insert(index, element.to_string());

        create_merkle_tree(&elements)
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
    pub fn get_aggregate_proof(
        ref_tree: &MerkleTree,
        start_index: usize,
        end_index: usize,
    ) -> Result<MerkleAggregateProof, String> {
        if start_index >= end_index || end_index >= ref_tree.leaves.len() {
            return Err(
                "Invalid range indices for the target elements.\
                 Ensure your start and end both fall within the leaves vector for the given tree."
                    .to_string(),
            );
        }

        let elements = ref_tree.leaves[start_index..end_index].to_vec();
        let mut siblings: Vec<String> = Vec::new();
        let mut directions: Vec<bool> = Vec::new();

        let mut current_row: Vec<MerkleNode> = ref_tree
            .leaves
            .to_owned()
            .iter()
            .map(|leaf| leaf.to_owned().into())
            .collect::<_>();
        let mut current_start = start_index;
        let mut current_end = end_index - 1;

        while current_start != 0 && current_end != (current_row.len() - 1) {
            let start_sibling_is_left_child = !current_start % 2 == 0;
            let end_sibling_is_right_child = !current_end % 2 == 1;

            if start_sibling_is_left_child {
                siblings.push(current_row[current_start - 1].value.to_owned());
            } else {
                siblings.push(MerkleNode::default().value.to_owned())
            }

            directions.push(start_sibling_is_left_child);

            if end_sibling_is_right_child {
                siblings.push(current_row[current_end + 1].value.to_owned());
            } else {
                siblings.push(MerkleNode::default().value.to_owned())
            }

            directions.push(end_sibling_is_right_child);

            current_row = generate_parent_row(current_row);
            current_start /= 2;
            current_end /= 2;
        }

        Ok(MerkleAggregateProof {
            elements,
            siblings,
            directions,
        })
    }

    pub fn verify_aggregate_proof(root: String, proof: &MerkleAggregateProof) -> bool {
        let mut current_row = proof
            .elements
            .iter()
            .map(|leaf| leaf.to_owned().into())
            .collect::<Vec<_>>();

        let proof_siblings = proof
            .siblings
            .iter()
            .zip(proof.directions.iter())
            .collect::<Vec<_>>();

        for chunk in proof_siblings.chunks(2) {
            println!("current row: {current_row:#?}");
            println!("chunk: {chunk:#?}");
            let (start_sibling, start_is_left_child) = chunk[0];
            let (end_sibling, end_is_right_child) = chunk[1];

            if *start_is_left_child {
                current_row.insert(
                    0,
                    MerkleNode {
                        value: start_sibling.to_owned(),
                        left: None,
                        right: None,
                    },
                );
            }

            if *end_is_right_child {
                current_row.push(MerkleNode {
                    value: end_sibling.to_owned(),
                    left: None,
                    right: None,
                });
            }

            current_row = generate_parent_row(current_row);
        }

        while current_row.len() > 1 {
            println!("current row: {current_row:#?}");
            current_row = generate_parent_row(current_row);
        }

        println!("root: {current_row:#?}");
        current_row[0].value.eq(&root)
    }
}

#[cfg(test)]
mod validations {
    use crate::merkle_tree::*;

    const TEST_ELEMENTS: [&str; 3] = ["some", "test", "elements"];
    const MORE_TEST_ELEMENTS: [&str; 4] = ["some", "more", "test", "elements"];
    const EVEN_MORE_TEST_ELEMENTS: [&str; 5] = ["some", "more", "valid", "test", "elements"];
    const YET_MORE_TEST_ELEMENTS: [&str; 6] = ["some", "more", "valid", "test", "elements", "too"];
    const LOTS_MORE_TEST_ELEMENTS: [&str; 7] =
        ["some", "more", "valid", "test", "elements", "to", "use"];
    const INCREASINGLY_MORE_TEST_ELEMENTS: [&str; 8] = [
        "some", "more", "valid", "test", "elements", "to", "use", "again",
    ];
    const INVALID_HASH: &str = "not_a_valid_hash";
    const VERIFY_PROOF_FAILED: bool = false;

    fn get_test_tree(input: Vec<&str>) -> MerkleTree {
        let elements = input.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        create_merkle_tree(&elements)
            .expect("Should have received a valid tree given const test inputs")
    }

    fn get_expected_root_hash(input: Vec<&str>) -> String {
        let mut leaves = input;
        if leaves.len() % 2 == 1 {
            leaves.push("");
        }

        let mut nodes: Vec<String> = leaves.iter().map(|e| hash_leaf(e)).collect::<_>();

        while nodes.len() > 1 {
            let head: Vec<String> = nodes
                .chunks_exact(2)
                .map(|pair| hash_node(&pair[0], &pair[1]))
                .collect::<_>();

            let tail: Vec<String> = nodes
                .chunks_exact(2)
                .remainder()
                .iter()
                .map(|s| hash_node(s, ""))
                .collect::<_>();

            nodes = head.into_iter().chain(tail.into_iter()).collect::<Vec<_>>();
        }

        nodes[0].to_owned()
    }

    #[test]
    fn getting_root_hashes() {
        let mt = get_test_tree(TEST_ELEMENTS.to_vec());
        let expected_root_hash = mt.root_hash.to_owned();

        assert_eq!(get_root(&mt), expected_root_hash);
    }

    #[test]
    fn verifying_proofs() {
        let mt = get_test_tree(TEST_ELEMENTS.to_vec());

        let proof =
            get_proof(&mt, 0).expect("Should have received a valid proof for the first element");

        assert!(verify_proof(get_root(&mt), &proof));
        assert_eq!(
            verify_proof(INVALID_HASH.into(), &proof),
            VERIFY_PROOF_FAILED
        );
    }

    #[test]
    fn updating_elements() {
        let mt = get_test_tree(TEST_ELEMENTS.to_vec());
        let new_element = "extra";
        let mut elements = TEST_ELEMENTS.to_vec();
        elements.insert(1, new_element);
        let expected_root = get_expected_root_hash(elements);

        let updated_mt = update_element(mt, 1, new_element).expect(
            "Should have received a valid tree from the implementation given these known inputs",
        );

        assert_eq!(get_root(&updated_mt), expected_root);
    }

    #[test]
    fn updating_elements_out_of_bounds() {
        let mt = get_test_tree(TEST_ELEMENTS.to_vec());
        let new_element = "and this is what it means to go even further beyond!";

        let oob = mt.leaves.len();
        let result = update_element(mt, oob, new_element);

        assert!(result.is_err());
    }

    #[test]
    fn generating_trees_of_varying_heights() {
        let mt_expected_root = get_expected_root_hash(TEST_ELEMENTS.to_vec());
        let mt = get_test_tree(TEST_ELEMENTS.to_vec());

        let more_mt_expected_root = get_expected_root_hash(MORE_TEST_ELEMENTS.to_vec());
        let more_mt = get_test_tree(MORE_TEST_ELEMENTS.to_vec());

        let even_more_mt_expected_root = get_expected_root_hash(EVEN_MORE_TEST_ELEMENTS.to_vec());
        let even_more_mt = get_test_tree(EVEN_MORE_TEST_ELEMENTS.to_vec());

        let yet_more_mt_expected_root = get_expected_root_hash(YET_MORE_TEST_ELEMENTS.to_vec());
        let yet_more_mt = get_test_tree(YET_MORE_TEST_ELEMENTS.to_vec());

        let lots_more_mt_expected_root = get_expected_root_hash(LOTS_MORE_TEST_ELEMENTS.to_vec());
        let lots_more_mt = get_test_tree(LOTS_MORE_TEST_ELEMENTS.to_vec());

        let increasingly_more_mt_expected_root =
            get_expected_root_hash(INCREASINGLY_MORE_TEST_ELEMENTS.to_vec());
        let increasingly_more_mt = get_test_tree(INCREASINGLY_MORE_TEST_ELEMENTS.to_vec());

        assert_eq!(get_root(&mt), mt_expected_root);
        assert_eq!(get_root(&more_mt), more_mt_expected_root);
        assert_eq!(get_root(&even_more_mt), even_more_mt_expected_root);
        assert_eq!(get_root(&yet_more_mt), yet_more_mt_expected_root);
        assert_eq!(get_root(&lots_more_mt), lots_more_mt_expected_root);
        assert_eq!(
            get_root(&increasingly_more_mt),
            increasingly_more_mt_expected_root
        );
    }

    #[test]
    fn verifying_aggregate_proofs() {
        let mt = get_test_tree(INCREASINGLY_MORE_TEST_ELEMENTS.to_vec());

        let proof = get_aggregate_proof(&mt, 2, 6)
            .expect("Should have received a valid proof for the elements [2,6)");

        assert!(verify_aggregate_proof(get_root(&mt), &proof));
        assert_eq!(
            verify_aggregate_proof(INVALID_HASH.into(), &proof),
            VERIFY_PROOF_FAILED
        );
    }

    #[test]
    fn verifying_aggregate_proofs_out_of_bounds() {
        let mt = get_test_tree(INCREASINGLY_MORE_TEST_ELEMENTS.to_vec());

        let oob = mt.leaves.len();
        let overflow_result = get_aggregate_proof(&mt, 0, oob);
        let invert_result = get_aggregate_proof(&mt, 1, 0);
        let eq_result = get_aggregate_proof(&mt, 2, 2);

        assert!(overflow_result.is_err());
        assert!(invert_result.is_err());
        assert!(eq_result.is_err());
    }

    #[test]
    fn test_root() {
        let expected_root = get_expected_root_hash(TEST_ELEMENTS.to_vec());

        let mt = get_test_tree(TEST_ELEMENTS.to_vec());

        assert_eq!(get_root(&mt), expected_root);
    }

    #[test]
    fn test_proof() {
        let mt = get_test_tree(TEST_ELEMENTS.to_vec());

        for i in 0..TEST_ELEMENTS.len() {
            let proof = get_proof(&mt, i)
                .expect("Should have received a valid proof for any of the original elements");

            assert!(verify_proof(get_root(&mt), &proof))
        }
    }
}
