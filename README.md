# Protochain

This is a project for prototyping a makeshift blockchain which uses the DeltaDB design for storing state.

Primary goals of project:

* Measure the overhead involved with DeltaDB with and without rent activated
* Measure differences caused by various parameter tweaks
* Measure the burden of proofs of SPV style clients on verifying data within DeltaDB
* Measure performance characteristics of key lookup and modification, as would be common with smart contracts


# Structures

Block header contents:

* previous hash
* deltadb root
* contents tree root


Block contents:

* header
* content[]

Content "transaction":

* address
* key
* value
* copy-from-address (optional)
* copy-from-key (optional)


Each content transaction will simply set a key within the specified address namespace to be a value. There is also the option to copy data from an address/key to the specified address and key (in which case value is not used)

There is no real authentication of anything in this blockchain as it is only intended for mocking up and performing practical tests on the overhead and difficulties encountered with the design of DeltaDB. 







