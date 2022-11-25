pragma circom 2.0.3;

include "../circom-pairing/circuits/bls_signature.circom";
include "../circom-pairing/circuits/curve.circom";
include "../circom-pairing/circuits/bls12_381_func.circom";

/**
 * Computes an aggregate BLS12-381 public key over a set of public keys and a bitmask
 * @param  b          The size of the set of public keys
 * @param  n          The number of bits to use per register
 * @param  k          The number of registers
 * @input  pubkeys    The b BLS12-381 public keys in BigInt(n, k)
 * @input  pubkeybits The b-length bitmask for which pubkeys to include
 * @output out        \sum_{i=0}^{b-1} pubkeys[i] * pubkeybits[i] (over the BLS12-381 curve)
 */
template AccumulatedECCAdd(b, n, k) {
    var p[50] = get_BLS12_381_prime(55, 7);

    signal input pubkeys[b][2][k];
    signal input pubkeybits[b];

    signal output out[2][k];

    component has_prev_nonzero[b];
    has_prev_nonzero[0] = OR();
    has_prev_nonzero[0].a <== 0;
    has_prev_nonzero[0].b <== pubkeybits[0];
    for (var i = 1; i < b; i++) {
        has_prev_nonzero[i] = OR();
        has_prev_nonzero[i].a <== has_prev_nonzero[i - 1].out;
        has_prev_nonzero[i].b <== pubkeybits[i];
    }

    signal partial[b][2][k];
    for (var idx = 0; idx < k; idx++) {
        for (var l = 0; l < 2; l++) {
            partial[0][l][idx] <== pubkeys[0][l][idx];
        }
    }

    component adders[b - 1];
    signal intermed1[b - 1][2][k];
    signal intermed2[b - 1][2][k];
    for (var i = 1; i < b; i++) {
        adders[i - 1] = EllipticCurveAddUnequal(n, k, p);
        for (var idx = 0; idx < k; idx++) {
            for (var l = 0; l < 2; l++) {
                adders[i - 1].a[l][idx] <== partial[i - 1][l][idx];
                adders[i - 1].b[l][idx] <== pubkeys[i][l][idx];
            }
        }


        for (var idx = 0; idx < k; idx++) {
            for (var l = 0; l < 2; l++) {
                intermed1[i - 1][l][idx] <== (1-pubkeybits[i]) * (partial[i - 1][l][idx] - adders[i - 1].out[l][idx]) + adders[i - 1].out[l][idx];
                intermed2[i - 1][l][idx] <== pubkeys[i][l][idx] - (1-pubkeybits[i]) * pubkeys[i][l][idx];
                partial[i][l][idx] <== has_prev_nonzero[i - 1].out * (intermed1[i - 1][l][idx] - intermed2[i - 1][l][idx]) + intermed2[i - 1][l][idx];
            }
        }
    }

    for (var idx = 0; idx < k; idx++) {
        for (var l = 0; l < 2; l++) {
            out[l][idx] <== partial[b - 1][l][idx];
        }
    }
}


/**
 * Verifies a BLS12-381 signature over a message hash and an aggregated pubkey
 * @param  b          The size of the set of public keys
 * @param  n          The number of bits to use per register
 * @param  k          The number of registers
 * @input  pubkeys    The b BLS12-381 public keys in BigInt(n, k)
 * @input  pubkeybits The b-length bitmask for which pubkeys to include
 * @input  signature  The BLS12-381 signature over the message hash
 * @input  Hm         The message hash (in field)
 */
template AggregateVerify(b, n, k){
    signal input pubkeys[b][2][k];
    signal input pubkeybits[b];
    signal input signature[2][2][k];
    signal input Hm[2][2][k];

    component aggregateKey = AccumulatedECCAdd(b,n,k);
    for (var batch_idx = 0; batch_idx < b; batch_idx++) {
        aggregateKey.pubkeybits[batch_idx] <== pubkeybits[batch_idx];
        for (var reg_idx = 0; reg_idx < k; reg_idx++) {
            for (var x_or_y = 0; x_or_y < 2; x_or_y++) {
                aggregateKey.pubkeys[batch_idx][x_or_y][reg_idx] <== pubkeys[batch_idx][x_or_y][reg_idx];
            }
        }
    }

    component verifySignature = CoreVerifyPubkeyG1(n, k);
    for (var reg_idx = 0; reg_idx < k; reg_idx++) {
        for (var x_or_y = 0; x_or_y < 2; x_or_y++) {
            verifySignature.pubkey[x_or_y][reg_idx] <== aggregateKey.out[x_or_y][reg_idx];
            log(aggregateKey.out[x_or_y][reg_idx]);
            verifySignature.signature[0][x_or_y][reg_idx] <== signature[0][x_or_y][reg_idx];
            verifySignature.signature[1][x_or_y][reg_idx] <== signature[1][x_or_y][reg_idx];
            verifySignature.hash[0][x_or_y][reg_idx] <== Hm[0][x_or_y][reg_idx];
            verifySignature.hash[1][x_or_y][reg_idx] <== Hm[1][x_or_y][reg_idx];
        }
    }
}


