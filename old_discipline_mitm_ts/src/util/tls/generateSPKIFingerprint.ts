import _ from "lodash"
import forge from "node-forge"
import {Buffer} from "node:buffer"


export type PEM = (
  | string 
  | string[] 
  | Buffer 
  | Buffer[]
)


const { pki, md, util: { encode64 } } = forge


const generateSPKIFingerprint = (certPem: PEM) => {
  let cert = pki.certificateFromPem(certPem.toString("utf8"))
	
  return encode64(
    pki.getPublicKeyFingerprint(cert.publicKey, {
			md: md.sha256.create(),
      type: "SubjectPublicKeyInfo",
      encoding: "binary",
    })
  )
}


export default generateSPKIFingerprint
export {generateSPKIFingerprint}