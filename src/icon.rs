use crate::entry::Entry;

/// Retourne l'icône Unicode (Nerd Font) correspondant au type ou à l'extension du fichier.
pub fn icon_for(entry: &Entry) -> char {
    if entry.is_dir     { return '\u{f024b}'; }  // 󰉋 folder
    if entry.is_symlink { return '\u{f0339}'; }  // 󰌹 symlink

    match entry.name.rsplit_once('.') {
        Some((_, ext)) => match ext {
            // DEV — langages
            "rs"                                        => '\u{f1617}', // 󱘗
            "py" | "pyw" | "pyi"                        => '\u{e73c}',  //
            "js" | "mjs" | "cjs"                        => '\u{ed0d}',  //
            "ts" | "mts" | "cts"                        => '\u{e628}',  //
            "jsx"                                       => '\u{e7ba}',  //
            "tsx"                                       => '\u{e7ba}',  //
            "c" | "h"                                   => '\u{e61e}',  //
            "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx"=> '\u{e61d}',  //
            "go"                                        => '\u{e626}',  //
            "java"                                      => '\u{e738}',  //
            "kt" | "kts"                                => '\u{e634}',  //
            "swift"                                     => '\u{e755}',  //
            "rb"                                        => '\u{e791}',  //
            "php"                                       => '\u{e73d}',  //
            "cs"                                        => '\u{f031b}', // 󰌛
            "fs" | "fsi" | "fsx"                        => '\u{e7a7}',  //
            "ex" | "exs"                                => '\u{e62d}',  //
            "hs" | "lhs"                                => '\u{e777}',  //
            "lua"                                       => '\u{e620}',  //
            "vim" | "vimrc"                             => '\u{e62b}',  //
            "sh" | "bash" | "zsh" | "fish" | "ksh"     => '\u{e691}',  //
            "ps1" | "psm1" | "psd1"                     => '\u{ebc7}',  //
            "zig"                                       => '\u{e6a9}',  //
            "nim"                                       => '\u{e677}',  //
            "dart"                                      => '\u{e798}',  //
            "r" | "rmd"                                 => '\u{f059f}', // 󰖟  (approx)
            "scala" | "sc"                              => '\u{e737}',  //
            "clj" | "cljs" | "cljc"                     => '\u{e76a}',  //
            "erl" | "hrl"                               => '\u{e7b1}',  //
            "ml" | "mli"                                => '\u{e67a}',  //
            "asm" | "s" | "nasm"                        => '\u{f471}',  //

            // DEV — web
            "html" | "htm"                              => '\u{e736}',  //
            "css"                                       => '\u{e749}',  //
            "scss" | "sass"                             => '\u{e603}',  //
            "less"                                      => '\u{e60b}',  //
            "vue"                                       => '\u{e6a0}',  //
            "svelte"                                    => '\u{e697}',  //
            "wasm"                                      => '\u{e6a9}',  //

            // DEV — data / config
            "json" | "jsonc"                            => '\u{e60b}',  //
            "yaml" | "yml"                              => '\u{e6a8}',  //
            "toml"                                      => '\u{e6b2}',  //
            "xml"                                       => '\u{f05c0}', // 󰗀
            "csv" | "tsv"                               => '\u{f0142}', //
            "sql"                                       => '\u{e706}',  //
            "graphql" | "gql"                           => '\u{e662}',  //
            "env"                                       => '\u{f462}',  //
            "ini" | "cfg" | "conf"                      => '\u{e615}',  //
            "editorconfig"                              => '\u{e615}',  //

            // DEV — build / CI
            "mk"                                        => '\u{e673}',  //
            "cmake"                                     => '\u{e673}',  //
            "dockerfile"                                => '\u{e650}',  //
            "gradle"                                    => '\u{e660}',  //
            "lock"                                      => '\u{f023}',  //
            "nix"                                       => '\u{f313}',  //

            // DEV — docs / texte
            "md" | "mdx" | "markdown"                   => '\u{e73e}',  //
            "rst"                                       => '\u{f15c}',  //
            "tex" | "bib"                               => '\u{e69b}',  //
            "txt"                                       => '\u{f0219}', // 󰈙
            "log"                                       => '\u{f0331}', // 󰌱
            "diff" | "patch"                            => '\u{e728}',  //

            // DEV — compilés / artefacts
            "o" | "a" | "so" | "dylib"                  => '\u{f471}',  //
            "class"                                     => '\u{e738}',  //
            "pyc" | "pyo"                               => '\u{e73c}',  //
            "whl" | "egg"                               => '\u{e73c}',  //
            "jar" | "war" | "ear"                       => '\u{e738}',  //

            // média — images
            "jpg" | "jpeg" | "jfif"                     => '\u{f03e}',  //  (photo)
            "png" | "apng"                              => '\u{f03e}',  //
            "gif"                                       => '\u{f03e}',  //
            "webp"                                      => '\u{f03e}',  //
            "bmp" | "tiff" | "tif"                      => '\u{f03e}',  //
            "svg" | "svgz"                              => '\u{fc1f}',  // (vectoriel)
            "ico"                                       => '\u{f03e}',  //
            "psd" | "psb"                               => '\u{f03e}',  // Photoshop
            "xcf"                                       => '\u{f03e}',  // GIMP
            "raw" | "cr2" | "cr3" | "nef" |
            "arw" | "dng" | "orf"                       => '\u{f03e}',  // RAW photo
            "heic" | "heif" | "avif"                    => '\u{f03e}',  //

            // média — vidéo
            "mp4" | "m4v"                               => '\u{f03d}',  //
            "mkv"                                       => '\u{f03d}',  //
            "avi"                                       => '\u{f03d}',  //
            "mov" | "qt"                                => '\u{f03d}',  //
            "webm"                                      => '\u{f03d}',  //
            "flv" | "f4v"                               => '\u{f03d}',  //
            "wmv" | "asf"                               => '\u{f03d}',  //
            "mpeg" | "mpg" | "m2v"                      => '\u{f03d}',  //
            "ogv"                                       => '\u{f03d}',  //
            "m2ts"                                      => '\u{f03d}',  // transport stream (ts/mts = TypeScript)
            "3gp" | "3g2"                               => '\u{f03d}',  //

            // média — audio
            "mp3"                                       => '\u{f001}',  //
            "flac"                                      => '\u{f001}',  //
            "ogg" | "oga"                               => '\u{f001}',  //
            "wav" | "wave"                              => '\u{f001}',  //
            "aac" | "m4a"                               => '\u{f001}',  //
            "opus"                                      => '\u{f001}',  //
            "wma"                                       => '\u{f001}',  //
            "aiff" | "aif"                              => '\u{f001}',  //
            "mid" | "midi"                              => '\u{f001}',  // MIDI
            "mka"                                       => '\u{f001}',  //
            "ape" | "wv"                                => '\u{f001}',  // lossless

            // archives
            "zip" | "tar" | "gz" | "bz2" |
            "xz" | "zst" | "rar" | "7z"                => '\u{f410}',  //

            // docs bureautique
            "pdf"                                       => '\u{f1c1}',  //
            "doc" | "docx" | "odt"                      => '\u{f1c2}',  //
            "xls" | "xlsx" | "ods"                      => '\u{f1c3}',  //
            "ppt" | "pptx" | "odp"                      => '\u{f1c4}',  //

            _                                           => '\u{f15b}',  //
        },
        None => '\u{f15b}',  //  fichier sans extension
    }
}
