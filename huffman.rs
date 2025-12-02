/**

Steps to build Huffman Tree
Input is an array of unique characters along with their frequency
of occurrences and output is Huffman Tree.

1. Create a leaf node for each unique character and build a min heap of all leaf nodes
(Min Heap is used as a priority queue. The value of frequency field is used to compare two nodes in min heap.
 Initially, the least frequent character is at root)
2. Extract two nodes with the minimum frequency from the min heap.
3. Create a new internal node with a frequency equal to the sum of the two nodes frequencies.
Make the first extracted node as its left child and the other extracted node as its right child. Add this node to the min heap.
4. Repeat steps#2 and #3 until the heap contains only one node. The remaining node is the root node and the tree is complete.


input: This is a test for the huffman coding

character      frequency
t                   4
i                   3
s                   3
f                   3
h                   2
e                   2
a                   2
o                   2
n                   2
r                   1
u                   1
m                   1
c                   1
d                   1
g                   1



Now need to buid a min-heap with 14 nodes, on node for each unique character

Build the tree from the leaf to the root!!

will get binary as a result, the binary needs to be in the same order as the phrase in order to decompress it


// What structures do we use to represent nodes a min-heap nodes?

**/
