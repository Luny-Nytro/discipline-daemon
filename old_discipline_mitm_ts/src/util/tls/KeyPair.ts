import forge from "node-forge"
import {KeyPair} from "./typings.js"


// We share a single keypair across all certificates in this process, and
// instantiate it once when the first CA is created, because it can be
// expensive (depending on the key length).
//
// This would be a terrible idea for a real server, but for a mock server
// it's ok - if anybody can steal this, they can steal the CA cert anyway.
let KEY_PAIR: KeyPair | undefined


function get() {
  return <KeyPair>KEY_PAIR
}

function set(keyLength: number) {
  // If we have no key, or not a long enough one, generate one.

  if (!KEY_PAIR || KEY_PAIR.length < keyLength) {
    const x = forge.pki.rsa.generateKeyPair(keyLength)

    KEY_PAIR = {
      length: keyLength,
      publicKey: x.publicKey, 
      privateKey: x.privateKey, 
    }
  }
}


export default Object.freeze({get, set})