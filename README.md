# Specular Technical Challenge

The test provides you an opportunity to demonstrate the following:
- Your ability to write a data structure algorithm (in this case a merkle tree).
- Your ability to write clean, idiomatic Go.

## Rationale
A core data structure in Ethereum is the merkle tree. It's a simple binary tree structure where the root node is represented by the hash of its two child hashes. Given any set of data in the leaves, this leads to a unique root. Furthermore, proof of existence of a piece of data can be represented by a hash path, a list of pairwise child hashes at each layer, from leaf to root.

In this test you will implement a basic version of a merkle tree.

## Merkle Tree Structure
- The merkle tree is from a list of elements and should have the minimum height needed to contain all elements
- Each node of the tree is computed by hashing the left and right subtree hashes with the provided `hashNode()` function 
- Once a tree is created with `NewMerkleTree()`, no new elements can be added
- Leafs can be strings of arbitrary length, empty Leafes should be represented by empty strings

## Building and Running
```
# clone the repo
git clone git@github.com:SpecularL2/interview-test.git

# run the test
go test .
```
