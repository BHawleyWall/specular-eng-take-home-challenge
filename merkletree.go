package merkletree

import (
	"crypto/sha256"
	"fmt"
)

// Verifies a Merkle proof against a known root.
func VerifyProof(root string, proof MerkleProof) bool {
	// TODO
	return true
}

// Hash function to be used for the construction of the merkle tree
func hashLeaf(leaf string) string {
	h := sha256.New()
	h.Write([]byte(leaf))
	return fmt.Sprintf("%x", h.Sum(nil))
}

// Hash function to be used for the construction of the merkle tree
func hashNode(a string, b string) string {
	h := sha256.New()
	h.Write([]byte(a))
	h.Write([]byte(b))
	return fmt.Sprintf("%x", h.Sum(nil))
}

type MerkleTree struct {
	// TODO
}

type MerkleProof struct {
	hElement   string   // hash of element for which we want to prove inclusion
	siblings   []string // path of siblings from the element up to the root
	directions []bool   // signal if the sibling at the same index is on the left or right
}

// Creates a merkle tree from a list of elements.
// The tree should have the minimum height needed to contain all elements.
// Empty slots should be filled with an empty string.
func NewMerkleTree(elements []string) (*MerkleTree, error) {
	// TODO
	return &MerkleTree{}, nil
}

func (t *MerkleTree) GetRoot() string {
	// TODO
	return ""
}

// Generates a Merkle proof of the inclusion of the element at the given index.
// If the index is out of bounds, an error is returned.
//
// Example:
// proof for index 2 (marked with `h`), return the nodes marked `*` at each layer.
//
// tree:
// d0:                                   [ R ]
// d1:                [*]                                     [*]
// d2:      [*]                 [*]                 [ ]                 [ ]
// d3: [ ]       [ ]       [h]       [*]       [ ]       [ ]       [ ]       [ ]
//
// proof:
// hElement   = h
// siblings   = [d3-3, d2-0, d1-1]
// directions = [false, true, false]
func (t *MerkleTree) GetProof(index uint64) (MerkleProof, error) {
	// TODO
	return MerkleProof{}, nil
}

// ** BONUS (optional - easy) **
// Updates the Merkle tree (from leaf to root) to include the new element at index.
// For simplicity, the index must be within the bounds of the original vector size.
// If it is not, return an error.
func (t *MerkleTree) UpdateElement(index uint64, element string) error {
	// TODO
	return nil
}

// ** BONUS (optional - hard) **
// Generates a Merkle proof of the inclusion of contiguous elements,
// starting at startIndex (inclusive) and ending at endIndex (exclusive).
// If the indexes are out of bounds or startIndex >= endIndex, an error is returned.
//
// Note: modify the method signature to return your proof type.
// Implement a separate VerifyAggregatedProof for this type.
//
// The aggregated proof size should generally be smaller than
// that of the naive approach (calling GetProof for every index).
func (t *MerkleTree) GetAggregatedProof(startIndex uint64, endIndex uint64) error {
	// TODO
	return nil
}
