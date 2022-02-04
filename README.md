# Hom

This is a silly personal investigation and proof of concept of a homomorphic "encryption", using the **monoid** structure of strings with the concatenation operation.

The "encryption" part is a simple permutation of the strings and the homomorphic property is characterized by being able to operate on the scrambled strings to produce a new scrambled one that when decrypted corresponds to the result of concatenating the original strings.