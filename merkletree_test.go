package merkletree

import (
	"fmt"
	"testing"
)

func TestRoot(t *testing.T) {
	elements := []string{"some", "test", "elements"}
	expectedRoot := "040c89dca6bd37584693bb94e6a68b6212edbc7f063d39b28ad6874dbd4f30d2"
	testname := fmt.Sprintf("computes correct root")
	t.Run(testname, func(t *testing.T) {
		mt := NewMerkleTree(elements)

		if mt.GetRoot() != expectedRoot {
			t.Errorf("got %s, want %s", mt.GetRoot(), expectedRoot)
		}
	})
}

func TestProof(t *testing.T) {
	elements := []string{"some", "test", "elements"}
	mt := NewMerkleTree(elements)

	for i, elem := range elements {
		testname := fmt.Sprintf("valid proof for element: %d", i)
		t.Run(testname, func(t *testing.T) {
			proof := mt.GetProof(uint64(i))

			if !VerifyProof(mt.GetRoot(), proof) {
				t.Error("invalid proof")
			}

			if hashLeaf(elem) != proof.element {
				t.Errorf("got %s, want %s", elem, proof.element)
			}
		})
	}
}
