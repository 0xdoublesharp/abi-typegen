import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { test } from "node:test";
import { SigningKey } from "ethers";
import { ethers as ethers5 } from "ethers5";

const require = createRequire(import.meta.url);
const legacyEthers = createRequire(require.resolve("ethers5"));
const signing = createRequire(
  legacyEthers.resolve("@ethersproject/signing-key"),
);
const { ec: EC } = signing("elliptic");

// Public Rooterberg vector 29; these are test data, never a funded key.
// https://github.com/bleichenbacher-daniel/Rooterberg/blob/main/test_vectors/ecdsa/ecdsa_rfc6979_secp521r1_sha_512.json
const vector = {
  key: "016e32d1a2635affa0e490052069e2be4189c2b9c1d55b46074aad54a354c3ddad231277d1b906fa44956a869e201c8ac4e63744f3a466c1e785453fcbb334d5f436",
  message: "bad5d34b8a3d4b5c67c2953a4ac8c675fb02",
  signature:
    "308187024201680b294d437694b2e1e6a89ab285e426557593c55f062244d20e86b12c33f1d09320bbd3a16ab6988b40e08698b223d74914bd9d6b60ac7c37eb289c12ba66f89302411bd2760f0cab608986367b13ba888363541e523cff0404954c1e872715c99857de68015ef12e3cfcd732ea04e9a61375ebb4b198229fea26be045970953734b064",
};

test("reported P-521 leading-zero nonce produces the RFC 6979 signature", () => {
  const curve = new EC("p521");
  const digest = createHash("sha512")
    .update(Buffer.from(vector.message, "hex"))
    .digest();
  const signature = curve.sign(digest, vector.key, "hex");
  assert.equal(signature.toDER("hex"), vector.signature);
  assert.equal(
    curve.verify(digest, signature, curve.keyFromPrivate(vector.key, "hex")),
    true,
  );
});

test("caller-supplied nonce behavior is preserved", () => {
  const curve = new EC("p521");
  const digest = createHash("sha512").update("public test message").digest();
  const nonce = new curve.n.constructor(3);
  const signature = curve.sign(digest, vector.key, {
    k(iteration) {
      assert.equal(
        iteration,
        0,
        "a valid supplied nonce must not be shifted or rejected",
      );
      return nonce;
    },
  });
  assert.equal(
    signature.r.toString(16),
    curve.g.mul(nonce).getX().umod(curve.n).toString(16),
  );
  assert.equal(
    curve.verify(digest, signature, curve.keyFromPrivate(vector.key, "hex")),
    true,
  );
});

test("Ethers v5 secp256k1 signatures match Ethers v6 for leading-zero digests", () => {
  const key = `0x${"1".padStart(64, "0")}`;
  const legacy = new ethers5.utils.SigningKey(key);
  const current = new SigningKey(key);
  for (const digest of [
    "0".repeat(64),
    "1".padStart(64, "0"),
    "0123456789abcdef".padStart(64, "0"),
  ]) {
    const input = `0x${digest}`;
    assert.equal(
      ethers5.utils.joinSignature(legacy.signDigest(input)),
      current.sign(input).serialized,
    );
  }
});
