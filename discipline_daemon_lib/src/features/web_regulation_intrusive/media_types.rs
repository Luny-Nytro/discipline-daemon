
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum MediaKind {
  /// Matches any image.
  Image,
  /// Matches any video.
  Video,
  /// Matches any font.
  Font,
  /// Matches any archive.
  Archive,
  /// Matches any HTML file or similar files.
  WebPage,
  /// Matches any CSS file or similar files.
  WebStyle,
  /// Matches any JavaScript file or similar files.
  WebScript,
  /// Matches any JavaScript file.
  JavaScript,
  /// Matches any TypeScript file.
  TypeScript,
  /// Matches any binary file or stream.
  Binary,
  /// Matches any shell script file.
  ShellScript,
  /// Any text file.
  Text,
  /// Any document (including text files).
  Document,
  /// Gif or similar images.
  GifLike,
  /// Matches any icon.
  Icon,
}