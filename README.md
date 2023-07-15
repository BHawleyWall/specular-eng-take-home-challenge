# Specular Technical Challenge

The test provides you an opportunity to demonstrate the following:
- Your ability to write a data structure algorithm (in this case a merkle tree).
- Your ability to write clean, idiomatic Go.

## Rationale
A core cryptographic primitive in Ethereum is the vector commitment, which commits to an ordered vector of elements and can be opened (i.e. proved that an element exists in the vector committed to) at any position. A Merkle tree is a vector commitment, represented by a simple binary tree structure where the root node is the hash of its two child hashes, concatenated. Given a collision-resistant hash function and any vector of elements represented by the leaves, this leads to a unique root. Proof of existence of an element can be represented by a hash path, a list of pairwise child hashes at each layer, from leaf to root.

In this test you will implement a basic version of a merkle tree.

## Merkle tree structure
- The merkle tree is constructed from a vector of elements, and should have the minimum height needed to contain all elements
- Each node of the tree is computed by hashing the left and right subtree hashes with the provided `hashNode()` function 
- Leaves are strings of arbitrary length; empty leaves should be represented by empty strings

## Building and Running
```
# clone the repo
git clone git@github.com:SpecularL2/interview-test.git

# run the test
go test .
```
