# BLS Library for unsafe signing

This library enables unsafe aggregate BLS signing, whereby all signers can sign the message. 
Note that this does not enforce that messages are distinct as a countermeasure against BLS's rogue-key attack.
See Section 3.1. of the IRTF's BLS signatures [spec](https://tools.ietf.org/html/draft-irtf-cfrg-bls-signature-02#section-3.1).
As such we need to use other methods to counter rogue-key attacks (eg. public key registration on a contract)

See `bls-utils` for some benchmarks and tests using this library. 

