pub struct Mime<'a> {
  pub name: &'a str,
  pub mime: &'a str,
  pub filename_extensions: &'a [&'a str],
}

static AUDIO_AAC: Mime<'static> = Mime {
  name: "Advanced Audio Coding",
  mime: "audio/acc",
  filename_extensions: &["acc"],
};

static APPLICATION_X_ABIWORD: Mime<'static> = Mime {
  name: "AbiWord Document",
  mime: "application/x-abiword",
  filename_extensions: &["abw"]
};

static APPLICATION_X_FREEARC: Mime<'static> = Mime {
  name: "FreeArc",
  mime: "application-x-freearc",
  filename_extensions: &["arc"],
};

static IMAGE_AVIF: Mime<'static> = Mime {
  name: "AV1 Image File Format",
  mime: "image/avif",
  filename_extensions: &["avif"],
};

static VIDEO_X_MSVIDEO: Mime<'static> = Mime {
  name: "Audio Video Interleave",
  mime: "video/x-msvideo",
  filename_extensions: &["avi"],
};

static APPLICATION_VND_AMAZON_EBOOK: Mime<'static> = Mime {
  name: "Amazon Kindle eBook Format",
  mime: "application/vnd.amazon.ebook",
  filename_extensions: &["azw"],
};

static APPLICATION_OCTET_STREAM: Mime<'static> = Mime {
  name: "Any Kind of Binary Data",
  mime: "application/octet-stream",
  filename_extensions: &["bin"],
};

static IMAGE_BMP: Mime<'static> = Mime {
  name: "Windows OS/2 Bitmap Graphics",
  mime: "image/bmp",
  filename_extensions: &["bmp"],
};

static APPLICATION_X_BZIP: Mime<'static> = Mime {
  name: "BZip archive",
  mime: "application/x-bzip",
  filename_extensions: &["bz"],
};

static APPLICATION_X_BZIP2: Mime<'static> = Mime {
  name: "BZip2 archive",
  mime: "application/x-bzip2",
  filename_extensions: &["bz2"],
};

static APPLICATION_X_CDF: Mime<'static> = Mime {
  name: "CD audio",
  mime: "application/x-cdf",
  filename_extensions: &["cda"],
};

static APPLICATION_X_CSH: Mime<'static> = Mime {
  name: "C-Shell script",
  mime: "application/x-csh",
  filename_extensions: &["csh"],
};

static TEXT_CSS: Mime<'static> = Mime {
  name: "Cascading Style Sheets (CSS)",
  mime: "text/css",
  filename_extensions: &["css"],
};

static TEXT_CSV: Mime<'static> = Mime {
  name: "Comma-Separated Values (CSV)",
  mime: "text/csv",
  filename_extensions: &["csv"],
};

static APPLICATION_MSWORD: Mime<'static> = Mime {
  name: "Microsoft Word",
  mime: "application/msword",
  filename_extensions: &["doc"],
};

static APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT: Mime<'static> = Mime {
  name: "Microsoft Word (OpenXML)",
  mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
  filename_extensions: &["docx"],
};

static APPLICATION_VND_MS_FONTOBJECT: Mime<'static> = Mime {
  name: "MS Embedded OpenType fonts",
  mime: "application/vnd.ms-fontobject",
  filename_extensions: &["eot"],
};

static APPLICATION_EPUB_ZIP: Mime<'static> = Mime {
  name: "Electronic publication (EPUB)",
  mime: "application/epub+zip",
  filename_extensions: &["epub"],
};

static APPLICATION_GZIP: Mime<'static> = Mime {
  name: "GZip Compressed Archive",
  mime: "application/gzip",
  filename_extensions: &["gz"],
};

static IMAGE_GIF: Mime<'static> = Mime {
  name: "Graphics Interchange Format (GIF)",
  mime: "image/gif",
  filename_extensions: &["gif"],
};

static TEXT_HTML: Mime<'static> = Mime {
  name: "HyperText Markup Language (HTML)",
  mime: "text/html",
  filename_extensions: &["htm", "html"],
};

static IMAGE_VND_MICROSOFT_ICON: Mime<'static> = Mime {
  name: "Icon format",
  mime: "image/vnd.microsoft.icon",
  filename_extensions: &["ico"],
};

static TEXT_CALENDAR: Mime<'static> = Mime {
  name: "iCalendar format",
  mime: "text/calendar",
  filename_extensions: &["ics"]
};

static APPLICATION_JAVA_ARCHIVE: Mime<'static> = Mime {
  name: "Java Archive (JAR)",
  mime: "application/java-archive",
  filename_extensions: &["jar"],
};

static IMAGE_JPEG: Mime<'static> = Mime {
  name: "JPEG images",
  mime: "image/jpeg",
  filename_extensions: &["jpeg", "jpg"],
};

// Per IETF RFC 9239 text/javascript is now standard and 
// application/javascript is now considered obsolete.
static APPLICATION_JAVASCRIPT: Mime<'static> = Mime {
  name: "JavaScript",
  mime: "application/javascript",
  filename_extensions: &["js"]
};

static APPLICATION_JSON: Mime<'static> = Mime {
  name: "JavaScript Object Notation (JSON) Format",
  mime: "application/json",
  filename_extensions: &["json"],
};

static APPLICATION_LD_JSON: Mime<'static> = Mime {
  name: "JSON-LD format",
  mime: "application/ld+json",
  filename_extensions: &["jsonld"],
};

static AUDIO_MIDI: Mime<'static> = Mime {
  name: "Musical Instrument Digital Interface (MIDI)",
  mime: "audio/midi",
  filename_extensions: &["mid", "midi"],
};

static AUDIO_X_MIDI: Mime<'static> = Mime {
  name: "Musical Instrument Digital Interface (MIDI)",
  mime: "audio/midi",
  filename_extensions: &["mid", "midi"],
};

// (Specifications: HTML and RFC 9239)
static TEXT_JAVASCRIPT: Mime<'static> = Mime {
  name: "JavaScript module",
  mime: "text/javascript",
  filename_extensions: &["js", "mjs"],
};

static AUDIO_MPEG: Mime<'static> = Mime {
  name: "MP3 audio",
  mime: "audio/mpeg",
  filename_extensions: &["mp3"],
};

static VIDEO_MP4: Mime<'static> = Mime {
  name: "MP4 video",
  mime: "video/mp4",
  filename_extensions: &["mp4"]
};

static VIDEO_MPEG: Mime<'static> = Mime {
  name: "MPEG Video",
  mime: "video/mpeg",
  filename_extensions: &["mpeg"],
};

static APPLICATION_VND_APPLE_INSTALLER_XML: Mime<'static> = Mime {
  name: "Apple Installer Package",
  mime: "application/vnd.apple.installer+xml",
  filename_extensions: &["mpkg"],
};

static APPLICATION_VND_OASIS_OPENDOCUMENT_PRESENTATION: Mime<'static> = Mime {
  name: "OpenDocument presentation document",
  mime: "application/vnd.oasis.opendocument.presentation",
  filename_extensions: &["odp"],
};

static APPLICATION_VND_OASIS_OPENDOCUMENT_SPREADSHEET: Mime<'static> = Mime {
  name: "OpenDocument spreadsheet document",
  mime: "application/vnd.oasis.opendocument.spreadsheet",
  filename_extensions: &["ods"],
};

static APPLICATION_VND_OASIS_OPENDOCUMENT_TEXT: Mime<'static> = Mime {
  name: "OpenDocument text document",
  mime: "application/vnd.oasis.opendocument.text",
  filename_extensions: &["odt"]
};

static AUDIO_OGG: Mime<'static> = Mime {
  name: "OGG audio",
  mime: "audio/ogg",
  filename_extensions: &["oga"],
};

static VIDEO_OGG: Mime<'static> = Mime {
  name: "OGG video",
  mime: "video/ogg",
  filename_extensions: &["ogv"],
};

static APPLICATION_OGG: Mime<'static> = Mime {
  // TODO(@me): Get more info about this to figure under which `MediaKind`"s it falls.
  name: "OGG",
  mime: "application/ogg",
  filename_extensions: &["ogx"],
};

static AUDIO_OPUS: Mime<'static> = Mime {
  name: "Opus audio",
  mime: "audio/opus",
  filename_extensions: &["opus"],
};

static FONT_OTF: Mime<'static> = Mime {
  name: "OpenType font",
  mime: "font/otf",
  filename_extensions: &["otf"],
};

static IMAGE_PNG: Mime<'static> = Mime {
  name: "Portable Network Graphics",
  mime: "image/png",
  filename_extensions: &["png"],
};

static APPLICATION_PDF: Mime<'static> = Mime {
  name: "Adobe Portable Document Format (PDF)",
  mime: "application/pdf",
  filename_extensions: &["pdf"],
};

static APPLICATION_X_HTTP_PHP: Mime<'static> = Mime {
  name: "Hypertext Preprocessor (Personal Home Page)",
  mime: "application/x-httpd-php",
  filename_extensions: &["php"],
};

static APPLICATION_VND_MS_POWERPOINT: Mime<'static> = Mime {
  name: "Microsoft PowerPoint",
  mime: "application/vnd.ms-powerpoint",
  filename_extensions: &["ppt"],
};

static APPLICATION_VND_OPENXMLFORMATS_OFFIECEDOCUMENT_PRESENTATIONML_PRESENTATION: Mime<'static> = Mime {
  name: "Microsoft PowerPoint (OpenXML)",
  mime: "application/vnd.openxmlformats-officedocument.presentationml.presentation",
  filename_extensions: &["pptx"],
};

static APPLICATION_VND_RAR: Mime<'static> = Mime {
  name: "RAR archive",
  mime: "application/vnd.rar",
  filename_extensions: &["rar"],
};

static APPLICATION_RTF: Mime<'static> = Mime {
  name: "Rich Text Format (RTF)",
  mime: "application/rtf",
  filename_extensions: &["rtf"],
};

static APPLICATION_X_SH: Mime<'static> = Mime {
  name: "Bourne shell script",
  mime: "application/x-sh",
  filename_extensions: &["sh"],
};

static IMAGE_SVG_XML: Mime<'static> = Mime {
  name: "Scalable Vector Graphics (SVG)",
  mime: "image/svg+xml",
  filename_extensions: &["svg"],
};

static APPLICATION_X_TAR: Mime<'static> = Mime {
  name: "Tape Archive (TAR)",
  mime: "application/x-tar",
  filename_extensions: &["tar"],
};

static IMAGE_TIFF: Mime<'static> = Mime {
  name: "Tagged Image File Format (TIFF)",
  mime: "image/tiff",
  filename_extensions: &["tif", ".tiff"],
};

static VIDEO_MP2T: Mime<'static> = Mime {
  name: "MPEG transport stream",
  mime: "video/mp2t",
  filename_extensions: &["ts"],
};

static FONT_TTF: Mime<'static> = Mime {
  name: "TrueType Font",
  mime: "font/ttf",
  filename_extensions: &["ttf"],
};

static TEXT_PLAIN: Mime<'static> = Mime {
  name: "Text, (generally ASCII or ISO 8859-n)",
  mime: "text/plain",
  filename_extensions: &["txt"],
};

static APPLICATION_VND_VISIO: Mime<'static> = Mime {
  name: "Microsoft Visio",
  mime: "application/vnd.visio",
  filename_extensions: &["vsd"],
};

static AUDIO_WAV: Mime<'static> = Mime {
  name: "Waveform Audio Format",
  mime: "audio/wav",
  filename_extensions: &["wav"],
};

static AUDIO_WEBM: Mime<'static> = Mime {
  name: "WebM audio",
  mime: "audio/webm",
  filename_extensions: &["weba"],
};

static VIDEO_WEBM: Mime<'static> = Mime {
  name: "WebM Video",
  mime: "video/webm",
  filename_extensions: &["webm"],
};

static IMAGE_WEBP: Mime<'static> = Mime {
  name: "WebP Image",
  mime: "image/webp",
  filename_extensions: &["webp"],
};

static FONT_WOFF: Mime<'static> = Mime {
  name: "Web Open Font Format",
  mime: "font/woff",
  filename_extensions: &["woff"],
};

static FONT_WOFF2: Mime<'static> = Mime {
  name: "Web Open Font Format 2",
  mime: "font/woff2",
  filename_extensions: &["woff2"],
};

static APPLICATION_XHTML_XML: Mime<'static> = Mime {
  name: "Extensible HyperText Markup Language",
  mime: "application/xhtml+xml",
  filename_extensions: &["xhtml"],
};

static APPLICATION_VND_MS_EXCEL: Mime<'static> = Mime {
  name: "Microsoft Excel",
  mime: "application/vnd.ms-excel",
  filename_extensions: &["xls"],
};

static APPLICATION_VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET: Mime<'static> = Mime {
  name: "Microsoft Excel (OpenXML)",
  mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  filename_extensions: &["xlsx"],
};

static APPLICATION_VND_MOZILLA_XUL_XML: Mime<'static> = Mime {
  name: "XML User Interface Language",
  mime: "application/vnd.mozilla.xul+xml",
  filename_extensions: &["xul"],
};

static APPLICATION_ZIP: Mime<'static> = Mime {
  name: "ZIP archive",
  mime: "application/zip",
  filename_extensions: &["zip"],
};

static VIDEO_3GPP: Mime<'static> = Mime {
  name: "3GPP video container",
  mime: "video/3gpp",
  filename_extensions: &["3gp"],
};

static AUDIO_3GPP: Mime<'static> = Mime {
  name: "3GPP audio container",
  mime: "audio/3gpp",
  filename_extensions: &["3gp"],
};

static VIDEO_3GPP2: Mime<'static> = Mime {
  name: "3GPP2 video container",
  mime: "video/3gpp2",
  filename_extensions: &["3g2"],
};

static AUDIO_3GPP2: Mime<'static> = Mime {
  name: "3GPP2 audio container",
  mime: "audio/3gpp2",
  filename_extensions: &["3g2"],
};

static APPLICATION_X_7Z_COMPRESSED: Mime<'static> = Mime {
  name: "7-zip archive",
  mime: "application/x-7z-compressed",
  filename_extensions: &["7z"],
};

// You can assign a specific MIME type to a file with .xml extension 
// depending on how its contents are meant to be interpreted. For instance, 
// an Atom feed is application/atom+xml, but application/xml serves as 
// a valid default. 
// this is recommended as of RFC 7303 (section 4.1), 
static APPLICATION_XML: Mime<'static> = Mime {
  name: "Extensible Markup Language",
  mime: "application/xml",
  filename_extensions: &["xml"],
};

// but this is still used sometimes. 
static TEXT_XML: Mime<'static> = Mime {
  name: "Extensible Markup Language",
  mime: "text/html",
  filename_extensions: &["xml"]
};




pub enum MediaKind {
  Audio,
  Video,
  Image,
  GIF,
}

pub fn moon<'a>() {
  let mimes = Vec::<Media<'a>>::new();

  let mut mime = Media {
    media_name: "Advanced Audio Coding",
    mimes: &["audio/aac"],
    filename_extensions: &["aac"],
    media_kinds: &[MediaKind::Audio]
  };
}