package merkletree

import (
	"fmt"
	"testing"
)

func TestRoot(t *testing.T) {
	elements := []string{"some", "test", "elements"}
	expectedRoot := hashNode(
		hashNode(hashLeaf("some"), hashLeaf("test")),
		hashNode(hashLeaf("elements"), hashLeaf("")),
	)
	testname := fmt.Sprintf("computes correct root")
	t.Run(testname, func(t *testing.T) {
		mt, err := NewMerkleTree(elements)
		if err != nil {
			t.Error(err)
		}
		if mt.GetRoot() != expectedRoot {
			t.Errorf("got %s, want %s", mt.GetRoot(), expectedRoot)
		}
	})
}

func TestProof(t *testing.T) {
	elements := []string{"some", "test", "elements"}
	mt, err := NewMerkleTree(elements)
	if err != nil {
		t.Error(err)
	}

	for i, elem := range elements {
		testname := fmt.Sprintf("valid proof for element: %d", i)
		t.Run(testname, func(t *testing.T) {
			proof, err := mt.GetProof(uint64(i))
			if err != nil {
				t.Error(err)
			}
			if !VerifyProof(mt.GetRoot(), proof) {
				t.Error("invalid proof")
			}
			if hashLeaf(elem) != proof.hElement {
				t.Errorf("got %s, want %s", elem, proof.hElement)
			}
		})
	}
}
