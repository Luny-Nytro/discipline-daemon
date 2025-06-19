import {v4} from "uuid"
import keys from "./KeyPair.js"
import forge from "node-forge"
import {PEM} from "./typings.js"
import {CertificateCache} from "./typings.js"
import {GeneratedCertificate} from "./typings.js"


export class CA {
  private caKey: forge.pki.PrivateKey
  private caCert: forge.pki.Certificate
  private certCache: CertificateCache


  constructor(caKey: PEM, caCert: PEM, keyLength: number = 2048) {
    this.caKey = forge.pki.privateKeyFromPem(caKey.toString("utf8"))
    this.caCert = forge.pki.certificateFromPem(caCert.toString("utf8"))
    this.certCache = {}
    keys.set(keyLength)
  }


  generateCertificate(domain: string): GeneratedCertificate {
    // TODO: Expire domains from the cache? Based on their actual expiry?
    if (this.certCache[domain]) {
      return this.certCache[domain]
    }

    let cert = forge.pki.createCertificate()

    cert.publicKey = keys.get().publicKey
    cert.serialNumber = v4().replace(/-/g, "")

    cert.validity.notBefore = new Date()
    // Make it valid for the last 24h - helps in cases where clocks slightly disagree.
    cert.validity.notBefore.setDate(cert.validity.notBefore.getDate() - 1)

    cert.validity.notAfter = new Date()
    // Valid for the next year by default. TODO: Shorten (and expire the cache) automatically.
    cert.validity.notAfter.setFullYear(cert.validity.notAfter.getFullYear() + 1)

    cert.setSubject([{ 
      name: "commonName", 
      value: domain 
    }, { 
      name: "organizationName", 
      value: "Antiweb",
    }])

    // make it self-signed
    cert.setIssuer(this.caCert.subject.attributes)

    cert.setExtensions([{
      name: "keyUsage",
      keyCertSign: true,
      nonRepudiation: true,
      keyEncipherment: true,
      dataEncipherment: true,
      digitalSignature: true,
    }, {
      name: "subjectAltName",
      altNames: [{
        type: 2,
        value: domain
      }]
    }])

    cert.sign(this.caKey, forge.md.sha256.create())

    const generatedCertificate = {
      ca: forge.pki.certificateToPem(this.caCert),
      key: forge.pki.privateKeyToPem(keys.get().privateKey),
      cert: forge.pki.certificateToPem(cert),
    }

    this.certCache[domain] = generatedCertificate

    return generatedCertificate
  }
}


export default CA