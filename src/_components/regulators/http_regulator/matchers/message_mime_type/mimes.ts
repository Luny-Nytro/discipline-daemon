/**!
 * This file contains information about the most common mime types on the web.
 * 
 * The information is taken from the following sources:
 *  - {@link https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types}
 * 
*/

import { Req } from "../../../Proxy.ts"

const acc = {
  name: 'Advanced Audio Coding',
  mimes: ['audio/aac'],
  extensions: ['.aac'],

  matchesExtension(extension: string) {
    return this.extensions[0] === extension
  },
  
  matchesMime(mime: string) {
    return this.mimes[0] === mime
  },
}

const abw = {
  name: 'AbiWord document',
  mimes: ['application/x-abiword'],
  extensions: ['.abw'],

  matchesExtension(extension: string) {
    return this.extensions[0] === extension
  },

  matchesMime(mime: string) {
    return this.mimes[0] === mime
  },
}

const arc = {
  name: 'Archive document (multiple files embedded)',
  mimes: ['application/x-freearc'],
  extensions: ['.arc'],

  matchesExtension(extension: string) {
    return this.extensions[0] === extension
  },

  matchesMime(mime: string) {
    return this.mimes[0] === mime
  },
}

const avif = {
  name: 'AVIF image',
  mimes: ['image/avif'],
  extensions: ['.avif'],
}

const avi = {
  name: 'AVI: Audio Video Interleave',
  mimes: ['video/x-msvideo'],
  extensions: ['.avi'],
}

const azw = {
  name: 'Amazon Kindle eBook format',
  mimes: ['application/vnd.amazon.ebook'],
  extensions: ['.azw'],
}

const bin = {
  name: 'Any kind of binary data',
  mimes: ['application/octet-stream'],
  extensions: ['.bin'],
}

const bmp = {
  name: 'Windows OS/2 Bitmap Graphics',
  mimes: ['image/bmp'],
  extensions: ['.bmp'],
}

const bz = {
  name: 'BZip archive',
  mimes: ['application/x-bzip'],
  extensions: ['.bz'],
}

const bz2 = {
  name: 'BZip2 archive',
  mimes: ['application/x-bzip2'],
  extensions: ['.bz2'],
}

const cda = {
  name: 'CD audio',
  mimes: ['application/x-cdf'],
  extensions: ['.cda'],
}

const csh = {
  name: 'C-Shell script',
  mimes: ['application/x-csh'],
  extensions: ['.csh'],
}

const css = {
  name: 'Cascading Style Sheets (CSS)',
  mimes: ['text/css'],
  extensions: ['.css'],
}

const csv = {
  name: 'Comma-separated values (CSV)',
  mimes: ['text/csv'],
  extensions: ['.csv'],
}

const doc = {
  name: 'Microsoft Word',
  mimes: ['application/msword'],
  extensions: ['.doc'],
}

const docx = {
  name: 'Microsoft Word (OpenXML)',
  mimes: ['application/vnd.openxmlformats-officedocument.wordprocessingml.document'],
  extensions: ['.docx'],
}

const eot = {
  name: 'MS Embedded OpenType fonts',
  mimes: ['application/vnd.ms-fontobject'],
  extensions: ['.eot'],
}

const epub = {
  name: 'Electronic publication (EPUB)',
  mimes: ['application/epub+zip'],
  extensions: ['.epub'],
}

const gz = {
  name: 'GZip Compressed Archive',
  mimes: ['application/gzip'],
  extensions: ['.gz'],
}

const gif = {
  name: 'Graphics Interchange Format (GIF)',
  mimes: ['image/gif'],
  extensions: ['.gif'],
}

const html = {
  name: 'HyperText Markup Language (HTML)',
  mimes: ['text/html'],
  extensions: ['.htm', '.html'],
}

const ico = {
  name: 'Icon format',
  mimes: ['image/vnd.microsoft.icon'],
  extensions: ['.ico'],
}

const ics = {
  name: 'iCalendar format',
  mimes: ['text/calendar'],
  extensions: ['.ics']
}

const jar = {
  name: 'Java Archive (JAR)',
  mimes: ['application/java-archive'],
  extensions: ['.jar'],
}

const jpg = {
  name: 'JPEG images',
  mimes: ['image/jpeg'],
  extensions: ['.jpeg', '.jpg'],
}

const js = {
  name: 'JavaScript',
  mimes: [
    // (Specifications: HTML and RFC 9239)
    'text/javascript',
    // Per IETF RFC 9239 text/javascript is now standard and 
    // application/javascript is now considered obsolete.
    'application/javascript'
  ],
  extensions: ['.js']
}

const json = {
  name: 'JSON format',
  mimes: ['application/json'],
  extensions: ['.json'],
}

const jsnold = {
  name: 'JSON-LD format',
  mimes: ['.jsonld'],
  extensions: ['application/ld+json'],
}

const midi = {
  name: 'Musical Instrument Digital Interface (MIDI)',
  mimes: ['audio/midi', 'audio/x-midi'],
  extensions: ['.mid', '.midi'],
}

const mjs = {
  name: 'JavaScript module',
  mimes: ['text/javascript'],
  extensions: ['.mjs'],
}

const mp3 = {
  name: 'MP3 audio',
  mimes: ['audio/mpeg'],
  extensions: ['.mp3'],
}

const mp4 = {
  name: 'MP4 video',
  mimes: ['video/mp4'],
  extensions: ['.mp4']
}

const mpeg = {
  name: 'MPEG Video',
  mimes: ['video/mpeg'],
  extensions: ['.mpeg'],
}

const mpkg = {
  name: 'Apple Installer Package',
  mimes: ['application/vnd.apple.installer+xml'],
  extensions: ['.mpkg'],
}

const odp = {
  name: 'OpenDocument presentation document',
  mimes: ['application/vnd.oasis.opendocument.presentation'],
  extensions: ['.odp'],
}

const ods = {
  name: 'OpenDocument spreadsheet document',
  mimes: ['application/vnd.oasis.opendocument.spreadsheet'],
  extensions: ['.ods'],
}

const odt = {
  name: 'OpenDocument text document',
  mimes: ['application/vnd.oasis.opendocument.text'],
  extensions: ['.odt']
}

const oga = {
  name: 'OGG audio',
  mimes: ['audio/ogg'],
  extensions: ['.oga'],
}

const ogv = {
  name: 'OGG video',
  mimes: ['video/ogg'],
  extensions: ['.ogv'],
}

const ogx = {
  // TODO(@me): Get more info about this to figure under which `MediaKind`'s it falls.
  name: 'OGG',
  mimes: ['application/ogg'],
  extensions: ['.ogx'],
}

const opus = {
  name: 'Opus audio',
  mimes: ['audio/opus'],
  extensions: ['.opus'],
}

const otf = {
  name: 'OpenType font',
  mimes: ['font/otf'],
  extensions: ['.otf'],
}

const png = {
  name: 'Portable Network Graphics',
  mimes: ['image/png'],
  extensions: ['.png'],
}

const pdf = {
  name: 'Adobe Portable Document Format (PDF)',
  mimes: ['application/pdf'],
  extensions: ['.pdf'],
}

const php = {
  name: 'Hypertext Preprocessor (Personal Home Page)',
  mimes: ['application/x-httpd-php'],
  extensions: ['.php'],
}

const ppt = {
  name: 'Microsoft PowerPoint',
  mimes: ['application/vnd.ms-powerpoint'],
  extensions: ['.ppt'],
}

const pptx = {
  name: 'Microsoft PowerPoint (OpenXML)',
  mimes: ['application/vnd.openxmlformats-officedocument.presentationml.presentation'],
  extensions: ['.pptx'],
}

const rar = {
  name: 'RAR archive',
  mimes: ['application/vnd.rar'],
  extensions: ['.rar'],
}

const rtf = {
  name: 'Rich Text Format (RTF)',
  mimes: ['application/rtf'],
  extensions: ['.rtf'],
}

const sh = {
  name: 'Bourne shell script',
  mimes: ['application/x-sh'],
  extensions: ['.sh'],
}

const svg = {
  name: 'Scalable Vector Graphics (SVG)',
  mimes: ['image/svg+xml'],
  extensions: ['.svg'],
}

const tar = {
  name: 'Tape Archive (TAR)',
  mimes: ['application/x-tar'],
  extensions: ['.tar'],
}

const tiff = {
  name: 'Tagged Image File Format (TIFF)',
  mimes: ['image/tiff'],
  extensions: ['.tif', '.tiff'],
}

const ts = {
  name: 'MPEG transport stream',
  mimes: ['video/mp2t'],
  extensions: ['.ts'],
}

const ttf = {
  name: 'TrueType Font',
  mimes: ['font/ttf'],
  extensions: ['.ttf'],
}

const txt = {
  name: 'Text, (generally ASCII or ISO 8859-n)',
  mimes: ['text/plain'],
  extensions: ['.txt'],
}

const vsd = {
  name: 'Microsoft Visio',
  mimes: ['application/vnd.visio'],
  extensions: ['.vsd'],
}

const wav = {
  name: 'Waveform Audio Format',
  mimes: ['audio/wav'],
  extensions: ['.wav'],
}

const weba = {
  name: 'WEBM audio',
  mimes: ['audio/webm'],
  extensions: ['.weba'],
}

const webm = {
  name: 'WEBM video',
  mimes: ['video/webm'],
  extensions: ['.webm'],
}

const webp = {
  name: 'WEBP image',
  mimes: ['image/webp'],
  extensions: ['.webp'],
}

const woff = {
  name: 'Web Open Font Format (WOFF)',
  mimes: ['font/woff'],
  extensions: ['.woff'],
}

const woff2 = {
  name: 'Web Open Font Format (WOFF)',
  mimes: ['font/woff2'],
  extensions: ['.woff2'],
}

const xhtml = {
  name: 'XHTML',
  mimes: ['application/xhtml+xml'],
  extensions: ['.xhtml'],
}

const xls = {
  name: 'Microsoft Excel',
  mimes: ['application/vnd.ms-excel'],
  extensions: ['.xls'],
}

const xlsx = {
  name: 'Microsoft Excel (OpenXML)',
  mimes: ['application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'],
  extensions: ['.xlsx'],
}

// You can assign a specific MIME type to a file with .xml extension 
// depending on how its contents are meant to be interpreted. For instance, 
// an Atom feed is application/atom+xml, but application/xml serves as 
// a valid default. 
const xml = {
  name: 'XML',
  mimes: [
    // this is recommended as of RFC 7303 (section 4.1), 
    'application/xml',
    // but this is still used sometimes. 
    'text/xml',
  ],
  extensions: ['.xml']
}

const xul = {
  name: 'XUL',
  mimes: ['application/vnd.mozilla.xul+xml'],
  extensions: ['.xul'],
}

const zip = {
  name: 'ZIP archive',
  mimes: ['application/zip'],
  extensions: ['.zip'],
}

const _3gpVideo = {
  name: '3GPP video container',
  mimes: ['video/3gpp'],
  extensions: ['.3gp'],
}

const _3gpAudio = {
  name: '3GPP audio container',
  mimes: ['audio/3gpp'],
  extensions: ['.3gp'],
}

const _3g2Video = {
  name: '3GPP2 video container',
  mimes: ['video/3gpp2'],
  extensions: ['.3g2'],
}
const _3g2Audio = {
  name: '3GPP2 audio container',
  mimes: ['audio/3gpp2'],
  extensions: ['.3g2'],
}

const _7z = {
  name: '7-zip archive',
  mimes: ['application/x-7z-compressed'],
  extensions: ['.7z'],
}

function isArchive(req: Req) {
  const header = req.headers.ContentType
  if (header) {
    return _7z.mimes.includes(header.mime)
      || zip.mimes.includes(header.mime)
  }
}