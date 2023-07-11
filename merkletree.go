package merkletree

import (
	"crypto/sha256"
	"fmt"
)

// hash function to be used for the construction of the merkle tree
func hashLeaf(leaf string) string {
	h := sha256.New()
	h.Write([]byte(leaf))
	return fmt.Sprintf("%x", h.Sum(nil))
}

// hash function to be used for the construction of the merkle tree
func hashNode(a string, b string) string {
	h := sha256.New()
	h.Write([]byte(a))
	h.Write([]byte(":"))
	h.Write([]byte(b))
	return fmt.Sprintf("%x", h.Sum(nil))
}

type MerkleTree struct {
	// TODO
}

type MerkleProof struct {
	element    string   // element for which we want to prove inclusion
	siblings   []string // path of siblings from the element up to the root
	directions []bool   // signal if the sibling at the same index is on the left or right
}

// create a merkle tree from a list of elements
// the tree should have the minimum height needed to contain all elements
// empty slots should be filled with an empty string
func NewMerkleTree(elements []string) *MerkleTree {
	// TODO
}

func (t *MerkleTree) GetRoot() string {
	// TODO
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
func (t *MerkleTree) GetProof(index uint64) MerkleProof {
	// TODO
}

// verify a merkle tree agains a known root
func VerifyProof(root string, proof MerkleProof) bool {
	// TODO
}
