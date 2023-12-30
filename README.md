# Summary
I went with what appears to be the bog-standard node struct and row-redux pattern (at least within C-style languages).

The Big-O time complexity of the `create_merkle_tree` function in this solution is O(_n_*log(_n_)). This complexity is derived from the iterative process of building the tree levels, where each level involves linear work (hashing and pairing) and the depth of the tree scales logarithmically with the number of leaves _n_.
The time complexity of the `get_proof` function is O(_n_). This is because the most time-consuming operations inside the while loop, which are the `position` method and the `generate_parent_row` function, collectively contribute to a linear complexity over the course of the logarithmic number of iterations.

There's definite improvements than can be made in both cases, but they would require deeper thought and exploration, and would possibly add more complexity to the code.  Without performance requirements driving such changes, I feel this strikes the 80/20 split correctly for an interview.

# Specular technical challenge

This challenge provides you an opportunity to demonstrate the following:
- Your ability to write a data structure algorithm (in this case a merkle tree).
- Your ability to write clean, idiomatic Go or Rust (choose one)

## Rationale
A core cryptographic primitive in Ethereum is the vector commitment, which commits to an ordered vector of elements and can be opened (i.e. proved that an element exists in the vector committed to) at any position. A Merkle tree is a vector commitment, represented by a simple binary tree structure where the root node is the hash of its two child hashes, concatenated. Given a collision-resistant hash function and any vector of elements represented by the leaves, this leads to a unique root. Proof of existence of an element can be represented by a hash path, a list of pairwise child hashes at each layer, from leaf to root.

In this test you will implement a basic version of a merkle tree.

### Merkle tree structure
- The merkle tree is constructed from a vector of elements, and should have the minimum height needed to contain all elements
- Each node of the tree is computed by hashing the left and right subtree hashes with the provided `hashNode()`/`hash_node()` and `hashLeaf()`/`hash_leaf()` function 
- Leaves represent the inclusion of strings of arbitrary length; empty leaves should assume empty strings

### Requirements
Implement the empty functions and make sure the tests pass. Also encouraged:
- Write some of your own tests, since those provided are fairly basic.
- Add a short summary at the top of the `README.md` to explain your solution and its asymptotic complexity.

## Building and running
```
# clone the repo
git clone git@github.com:SpecularL2/interview-test.git

# run the test for go
go test .

# run the tests for rust
cargo test
```

## Submission

We request that you avoid posting your solution publicly. Instead, preferably—upload your submission to a private repo and provide access via [Gitfront](https://gitfront.io/). Follow the website's instructions, or tldr here: (1) click `Get Started -> Create a new account -> Add repository`; (2) copy-paste the repo's SSH link from Github and click `Add`; (3) copy-paste the generated key from Gitfront to the deploy keys in the Github project, and (4) click `Build -> View`, and send us the link. 

This allows you to provide us an easy-to-access shareable link. Alternatively, if this workflow doesn't work for you just send us the files as a zip attachment (or via some other accessible means).
