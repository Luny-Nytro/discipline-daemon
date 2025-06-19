import uuid from "uuid"
import forge from "node-forge"

type GenerateCACertificateOpts = {
	bits?: number 
	commonName?: string
}

/**
 * Generate a CA certificate for mocking HTTPS.
 *
 * Returns a promise, for an object with key and cert properties,
 * containing the generated private key and certificate in PEM format.
 *
 * These can be saved to disk, and their paths passed
 * as HTTPS options to a Mockttp server.
 */
async function generateCACertificate(options: GenerateCACertificateOpts = {}) {
	const bits = options.bits ?? 2048
	const commonName = options.commonName ?? "Mockttp Testing CA - DO NOT TRUST - TESTING ONLY"


  const keyPair = await new Promise<forge.pki.rsa.KeyPair>((resolve, reject) => {
    forge.pki.rsa.generateKeyPair({ bits: options.bits }, (error, keyPair) => {
      if (error) reject(error)
      else resolve(keyPair)
    })
  })

  const cert = forge.pki.createCertificate()
  cert.publicKey = keyPair.publicKey
  cert.serialNumber = uuid.v4().replace(/-/g, '')

  cert.validity.notBefore = new Date()
  // Make it valid for the last 24h - helps in cases where clocks slightly disagree
  cert.validity.notBefore.setDate(cert.validity.notBefore.getDate() - 1)

  cert.validity.notAfter = new Date()
  // Valid for the next year by default.
  cert.validity.notAfter.setFullYear(cert.validity.notAfter.getFullYear() + 1)

  cert.setSubject([{ 
		name: "commonName", 
		value: options.commonName,
	}])

  cert.setExtensions([{
    name: 'basicConstraints',
    cA: true
  }])

  // Self-issued too
  cert.setIssuer(cert.subject.attributes)

  // Self-sign the certificate - we're the root
  cert.sign(keyPair.privateKey, forge.md.sha256.create())


  return {
    key: forge.pki.privateKeyToPem(keyPair.privateKey),
    cert: forge.pki.certificateToPem(cert),
  }
}


export {generateCACertificate}
export default generateCACertificate