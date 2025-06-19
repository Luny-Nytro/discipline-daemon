import forge from "node-forge"
import {Buffer} from "node:buffer"


export type PEM = (
  | string 
  | string[] 
  | Buffer 
  | Buffer[]
)

export type KeyPair = {
  publicKey: forge.pki.rsa.PublicKey
  privateKey: forge.pki.rsa.PrivateKey
  length: number
}

export type CAOptions = (
	| HttpsOptions 
	// | HttpsPathOptions
)

export type HttpsOptions = {
  key: string
  cert: string
  keyLength?: number
}

export type HttpsPathOptions = {
  keyPath: string
  certPath: string
  keyLength?: number
}

export type GeneratedCertificate = {
  ca: string
  key: string
  cert: string
}

export type CertificateCache = {
  [domain: string]: GeneratedCertificate 
}