use crate::entry::Entry;
use phf::phf_map;

static ICONS: phf::Map<&'static str, char> = phf_map! {
    // DEV — langages
    "rs"           => '\u{f1617}', // 󱘗 Rust
    "py"           => '\u{e73c}',  //  Python
    "pyw"          => '\u{e73c}',
    "pyi"          => '\u{e73c}',
    "js"           => '\u{ed0d}',  //  JavaScript
    "mjs"          => '\u{ed0d}',
    "cjs"          => '\u{ed0d}',
    "ts"           => '\u{e628}',  //  TypeScript
    "mts"          => '\u{e628}',
    "cts"          => '\u{e628}',
    "jsx"          => '\u{e7ba}',  //
    "tsx"          => '\u{e7ba}',
    "c"            => '\u{e61e}',  //  C
    "h"            => '\u{e61e}',
    "cpp"          => '\u{e61d}',  //  C++
    "cc"           => '\u{e61d}',
    "cxx"          => '\u{e61d}',
    "hpp"          => '\u{e61d}',
    "hh"           => '\u{e61d}',
    "hxx"          => '\u{e61d}',
    "go"           => '\u{e626}',  //  Go
    "java"         => '\u{e738}',  //  Java
    "kt"           => '\u{e634}',  //  Kotlin
    "kts"          => '\u{e634}',
    "swift"        => '\u{e755}',  //  Swift
    "rb"           => '\u{e791}',  //  Ruby
    "php"          => '\u{e73d}',  //  PHP
    "cs"           => '\u{f031b}', // 󰌛 C#
    "fs"           => '\u{e7a7}',  //  F#
    "fsi"          => '\u{e7a7}',
    "fsx"          => '\u{e7a7}',
    "ex"           => '\u{e62d}',  //  Elixir
    "exs"          => '\u{e62d}',
    "hs"           => '\u{e777}',  //  Haskell
    "lhs"          => '\u{e777}',
    "lua"          => '\u{e620}',  //  Lua
    "vim"          => '\u{e62b}',  //  Vim
    "vimrc"        => '\u{e62b}',
    "sh"           => '\u{e691}',  //  Shell
    "bash"         => '\u{e691}',
    "zsh"          => '\u{e691}',
    "fish"         => '\u{e691}',
    "ksh"          => '\u{e691}',
    "ps1"          => '\u{ebc7}',  //  PowerShell
    "psm1"         => '\u{ebc7}',
    "psd1"         => '\u{ebc7}',
    "zig"          => '\u{e6a9}',  //  Zig
    "nim"          => '\u{e677}',  //  Nim
    "dart"         => '\u{e798}',  //  Dart
    "r"            => '\u{f059f}', // 󰖟 R
    "rmd"          => '\u{f059f}',
    "scala"        => '\u{e737}',  //  Scala
    "sc"           => '\u{e737}',
    "clj"          => '\u{e76a}',  //  Clojure
    "cljs"         => '\u{e76a}',
    "cljc"         => '\u{e76a}',
    "erl"          => '\u{e7b1}',  //  Erlang
    "hrl"          => '\u{e7b1}',
    "ml"           => '\u{e67a}',  //  OCaml
    "mli"          => '\u{e67a}',
    "asm"          => '\u{f471}',  //  Assembly
    "s"            => '\u{f471}',
    "nasm"         => '\u{f471}',

    // DEV — web
    "html"         => '\u{e736}',  //  HTML
    "htm"          => '\u{e736}',
    "css"          => '\u{e749}',  //  CSS
    "scss"         => '\u{e603}',  //  SCSS
    "sass"         => '\u{e603}',
    "less"         => '\u{e60b}',  //  Less
    "vue"          => '\u{e6a0}',  //  Vue
    "svelte"       => '\u{e697}',  //  Svelte
    "wasm"         => '\u{e6a9}',  //  WebAssembly

    // DEV — data / config
    "json"         => '\u{e60b}',  //  JSON
    "jsonc"        => '\u{e60b}',
    "yaml"         => '\u{e6a8}',  //  YAML
    "yml"          => '\u{e6a8}',
    "toml"         => '\u{e6b2}',  //  TOML
    "xml"          => '\u{f05c0}', // 󰗀 XML
    "csv"          => '\u{f0142}', //  CSV
    "tsv"          => '\u{f0142}',
    "sql"          => '\u{e706}',  //  SQL
    "graphql"      => '\u{e662}',  //  GraphQL
    "gql"          => '\u{e662}',
    "env"          => '\u{f462}',  //  Env
    "ini"          => '\u{e615}',  //  Config
    "cfg"          => '\u{e615}',
    "conf"         => '\u{e615}',
    "editorconfig" => '\u{e615}',

    // DEV — build / CI
    "mk"           => '\u{e673}',  //  Makefile
    "cmake"        => '\u{e673}',
    "dockerfile"   => '\u{e650}',  //  Docker
    "gradle"       => '\u{e660}',  //  Gradle
    "lock"         => '\u{f023}',  //  Lock
    "nix"          => '\u{f313}',  //  Nix

    // DEV — docs / texte
    "md"           => '\u{e73e}',  //  Markdown
    "mdx"          => '\u{e73e}',
    "markdown"     => '\u{e73e}',
    "rst"          => '\u{f15c}',  //  reStructuredText
    "tex"          => '\u{e69b}',  //  LaTeX
    "bib"          => '\u{e69b}',
    "txt"          => '\u{f0219}', // 󰈙 Text
    "log"          => '\u{f0331}', // 󰌱 Log
    "diff"         => '\u{e728}',  //  Diff
    "patch"        => '\u{e728}',

    // DEV — compilés / artefacts
    "o"            => '\u{f471}',  //  Object
    "a"            => '\u{f471}',
    "so"           => '\u{f471}',
    "dylib"        => '\u{f471}',
    "class"        => '\u{e738}',  //  Java class
    "pyc"          => '\u{e73c}',  //  Python compiled
    "pyo"          => '\u{e73c}',
    "whl"          => '\u{e73c}',
    "egg"          => '\u{e73c}',
    "jar"          => '\u{e738}',  //  Java archive
    "war"          => '\u{e738}',
    "ear"          => '\u{e738}',

    // Média — images
    "jpg"          => '\u{f03e}',  //  Image
    "jpeg"         => '\u{f03e}',
    "jfif"         => '\u{f03e}',
    "png"          => '\u{f03e}',
    "apng"         => '\u{f03e}',
    "gif"          => '\u{f03e}',
    "webp"         => '\u{f03e}',
    "bmp"          => '\u{f03e}',
    "tiff"         => '\u{f03e}',
    "tif"          => '\u{f03e}',
    "svg"          => '\u{fc1f}',  //  SVG
    "svgz"         => '\u{fc1f}',
    "ico"          => '\u{f03e}',
    "psd"          => '\u{f03e}',  // Photoshop
    "psb"          => '\u{f03e}',
    "xcf"          => '\u{f03e}',  // GIMP
    "raw"          => '\u{f03e}',  // RAW photo
    "cr2"          => '\u{f03e}',
    "cr3"          => '\u{f03e}',
    "nef"          => '\u{f03e}',
    "arw"          => '\u{f03e}',
    "dng"          => '\u{f03e}',
    "orf"          => '\u{f03e}',
    "heic"         => '\u{f03e}',
    "heif"         => '\u{f03e}',
    "avif"         => '\u{f03e}',

    // Média — vidéo
    "mp4"          => '\u{f03d}',  //  Vidéo
    "m4v"          => '\u{f03d}',
    "mkv"          => '\u{f03d}',
    "avi"          => '\u{f03d}',
    "mov"          => '\u{f03d}',
    "qt"           => '\u{f03d}',
    "webm"         => '\u{f03d}',
    "flv"          => '\u{f03d}',
    "f4v"          => '\u{f03d}',
    "wmv"          => '\u{f03d}',
    "asf"          => '\u{f03d}',
    "mpeg"         => '\u{f03d}',
    "mpg"          => '\u{f03d}',
    "m2v"          => '\u{f03d}',
    "ogv"          => '\u{f03d}',
    "m2ts"         => '\u{f03d}',
    "3gp"          => '\u{f03d}',
    "3g2"          => '\u{f03d}',

    // Média — audio
    "mp3"          => '\u{f001}',  //  Audio
    "flac"         => '\u{f001}',
    "ogg"          => '\u{f001}',
    "oga"          => '\u{f001}',
    "wav"          => '\u{f001}',
    "wave"         => '\u{f001}',
    "aac"          => '\u{f001}',
    "m4a"          => '\u{f001}',
    "opus"         => '\u{f001}',
    "wma"          => '\u{f001}',
    "aiff"         => '\u{f001}',
    "aif"          => '\u{f001}',
    "mid"          => '\u{f001}',  // MIDI
    "midi"         => '\u{f001}',
    "mka"          => '\u{f001}',
    "ape"          => '\u{f001}',  // lossless
    "wv"           => '\u{f001}',

    // Archives
    "zip"          => '\u{f410}',  //  Archive
    "tar"          => '\u{f410}',
    "gz"           => '\u{f410}',
    "bz2"          => '\u{f410}',
    "xz"           => '\u{f410}',
    "zst"          => '\u{f410}',
    "rar"          => '\u{f410}',
    "7z"           => '\u{f410}',

    // Docs bureautique
    "pdf"          => '\u{f1c1}',  //  PDF
    "doc"          => '\u{f1c2}',  //  Word
    "docx"         => '\u{f1c2}',
    "odt"          => '\u{f1c2}',
    "xls"          => '\u{f1c3}',  //  Excel
    "xlsx"         => '\u{f1c3}',
    "ods"          => '\u{f1c3}',
    "ppt"          => '\u{f1c4}',  //  PowerPoint
    "pptx"         => '\u{f1c4}',
    "odp"          => '\u{f1c4}',
};

/// Retourne l'icône Unicode (Nerd Font) correspondant au type ou à l'extension du fichier.
pub fn icon_for(entry: &Entry) -> char {
    if entry.is_dir     { return '\u{f024b}'; }  // 󰉋 folder
    if entry.is_symlink { return '\u{f0339}'; }  // 󰌹 symlink

    let ext = entry.name.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
    *ICONS.get(ext).unwrap_or(&'\u{f15b}')  //
}
