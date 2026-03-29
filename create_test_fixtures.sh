#!/usr/bin/env bash
# Crée une arborescence de test reproductible pour ruls
# Usage: ./create_test_fixtures.sh

set -e

ROOT="tests/fixtures"

echo "Création des fixtures dans $ROOT..."
rm -rf "$ROOT"
mkdir -p "$ROOT"

# ----------------------------------------------------------
# Fichiers normaux — tailles variées
# ----------------------------------------------------------
mkdir -p "$ROOT/files"
touch "$ROOT/files/empty"                                             # 0 octets
echo "hello" > "$ROOT/files/tiny.txt"                                # ~6 octets
python3 -c "sys.stdout.buffer.write(b'x' * 512)" 2>/dev/null \
    || dd if=/dev/zero bs=512 count=1 2>/dev/null > "$ROOT/files/small_512.bin"
python3 -c "import sys; sys.stdout.buffer.write(b'x' * 1024)" \
    > "$ROOT/files/1k.bin"                                            # 1 KB
python3 -c "import sys; sys.stdout.buffer.write(b'x' * (100*1024))" \
    > "$ROOT/files/100k.bin"                                          # 100 KB
python3 -c "import sys; sys.stdout.buffer.write(b'x' * (1024*1024))" \
    > "$ROOT/files/1m.bin"                                            # 1 MB
python3 -c "import sys; sys.stdout.buffer.write(b'x' * (50*1024*1024))" \
    > "$ROOT/files/50m.bin"                                           # 50 MB
python3 -c "import sys; sys.stdout.buffer.write(b'x' * (512*1024*1024))" \
    > "$ROOT/files/512m.bin"                                          # 512 MB
# 1 GB — utilise dd pour éviter de charger 1GB en RAM
dd if=/dev/zero of="$ROOT/files/1g.bin" bs=1M count=1024 2>/dev/null # 1 GB
echo '{"key": "value"}' > "$ROOT/files/data.json"
echo 'fn main() { println!("hello"); }' > "$ROOT/files/main.rs"
echo '#!/usr/bin/env python3' > "$ROOT/files/script.py"
cp /bin/ls "$ROOT/files/executable_copy" 2>/dev/null || true         # binaire exécutable

# ----------------------------------------------------------
# Fichiers cachés
# ----------------------------------------------------------
echo "hidden content" > "$ROOT/.hidden_file"
echo "EDITOR=vim" > "$ROOT/.env"
mkdir -p "$ROOT/.hidden_dir"
echo "inside hidden dir" > "$ROOT/.hidden_dir/file.txt"

# ----------------------------------------------------------
# Dossiers imbriqués
# ----------------------------------------------------------
mkdir -p "$ROOT/dir_a/dir_b/dir_c"
echo "deep file" > "$ROOT/dir_a/dir_b/dir_c/deep.txt"
echo "level a" > "$ROOT/dir_a/a.txt"
echo "level b" > "$ROOT/dir_a/dir_b/b.txt"

# Dossier vide
mkdir -p "$ROOT/empty_dir"

# ----------------------------------------------------------
# Liens symboliques
# ----------------------------------------------------------
ln -sf "$ROOT/files/small.txt" "$ROOT/symlink_to_file"
ln -sf "$ROOT/dir_a" "$ROOT/symlink_to_dir"
ln -sf "/nonexistent/target" "$ROOT/broken_symlink"                  # lien cassé

# ----------------------------------------------------------
# Média
# ----------------------------------------------------------
mkdir -p "$ROOT/media/images"
touch "$ROOT/media/images/photo.jpg" "$ROOT/media/images/photo.jpeg" "$ROOT/media/images/scan.jfif"
touch "$ROOT/media/images/logo.png" "$ROOT/media/images/anim.apng"
touch "$ROOT/media/images/anim.gif"
touch "$ROOT/media/images/photo.webp"
touch "$ROOT/media/images/bitmap.bmp" "$ROOT/media/images/scan.tiff" "$ROOT/media/images/scan.tif"
touch "$ROOT/media/images/icon.svg" "$ROOT/media/images/icon.svgz"
touch "$ROOT/media/images/favicon.ico"
touch "$ROOT/media/images/design.psd" "$ROOT/media/images/design.psb"
touch "$ROOT/media/images/project.xcf"
touch "$ROOT/media/images/photo.raw" "$ROOT/media/images/canon.cr2" "$ROOT/media/images/canon.cr3"
touch "$ROOT/media/images/nikon.nef" "$ROOT/media/images/sony.arw" "$ROOT/media/images/adobe.dng" "$ROOT/media/images/olympus.orf"
touch "$ROOT/media/images/photo.heic" "$ROOT/media/images/photo.heif" "$ROOT/media/images/photo.avif"

mkdir -p "$ROOT/media/video"
touch "$ROOT/media/video/film.mp4" "$ROOT/media/video/film.m4v"
touch "$ROOT/media/video/film.mkv"
touch "$ROOT/media/video/film.avi"
touch "$ROOT/media/video/film.mov" "$ROOT/media/video/film.qt"
touch "$ROOT/media/video/film.webm"
touch "$ROOT/media/video/film.flv" "$ROOT/media/video/film.f4v"
touch "$ROOT/media/video/film.wmv" "$ROOT/media/video/film.asf"
touch "$ROOT/media/video/film.mpeg" "$ROOT/media/video/film.mpg" "$ROOT/media/video/film.m2v"
touch "$ROOT/media/video/film.ogv"
touch "$ROOT/media/video/stream.m2ts"
touch "$ROOT/media/video/film.3gp" "$ROOT/media/video/film.3g2"

mkdir -p "$ROOT/media/audio"
touch "$ROOT/media/audio/song.mp3"
touch "$ROOT/media/audio/song.flac"
touch "$ROOT/media/audio/song.ogg" "$ROOT/media/audio/song.oga"
touch "$ROOT/media/audio/song.wav" "$ROOT/media/audio/song.wave"
touch "$ROOT/media/audio/song.aac" "$ROOT/media/audio/song.m4a"
touch "$ROOT/media/audio/song.opus"
touch "$ROOT/media/audio/song.wma"
touch "$ROOT/media/audio/song.aiff" "$ROOT/media/audio/song.aif"
touch "$ROOT/media/audio/song.mid" "$ROOT/media/audio/song.midi"
touch "$ROOT/media/audio/song.mka"
touch "$ROOT/media/audio/song.ape" "$ROOT/media/audio/song.wv"

# ----------------------------------------------------------
# Extensions variées
# ----------------------------------------------------------
mkdir -p "$ROOT/types"
touch "$ROOT/types/image.png"
touch "$ROOT/types/image.jpg"
touch "$ROOT/types/archive.tar.gz"
touch "$ROOT/types/archive.zip"
touch "$ROOT/types/document.pdf"
touch "$ROOT/types/music.mp3"
touch "$ROOT/types/video.mp4"
touch "$ROOT/types/code.c"
touch "$ROOT/types/code.cpp"
touch "$ROOT/types/code.py"
touch "$ROOT/types/code.rs"
touch "$ROOT/types/Makefile"
touch "$ROOT/types/Dockerfile"

# ----------------------------------------------------------
# Extensions dev — une par type d'icône
# ----------------------------------------------------------
mkdir -p "$ROOT/dev"
# langages
touch "$ROOT/dev/main.rs"
touch "$ROOT/dev/script.py" "$ROOT/dev/types.pyi"
touch "$ROOT/dev/index.js" "$ROOT/dev/module.mjs"
touch "$ROOT/dev/app.ts" "$ROOT/dev/types.mts"
touch "$ROOT/dev/component.jsx"
touch "$ROOT/dev/component.tsx"
touch "$ROOT/dev/hello.c" "$ROOT/dev/header.h"
touch "$ROOT/dev/main.cpp" "$ROOT/dev/lib.hpp"
touch "$ROOT/dev/main.go"
touch "$ROOT/dev/Main.java"
touch "$ROOT/dev/Main.kt"
touch "$ROOT/dev/main.swift"
touch "$ROOT/dev/script.rb"
touch "$ROOT/dev/index.php"
touch "$ROOT/dev/Program.cs"
touch "$ROOT/dev/Main.fs"
touch "$ROOT/dev/main.ex" "$ROOT/dev/mix.exs"
touch "$ROOT/dev/Main.hs"
touch "$ROOT/dev/script.lua"
touch "$ROOT/dev/init.vim"
touch "$ROOT/dev/script.sh" "$ROOT/dev/config.zsh" "$ROOT/dev/config.fish"
touch "$ROOT/dev/script.ps1"
touch "$ROOT/dev/main.zig"
touch "$ROOT/dev/main.nim"
touch "$ROOT/dev/main.dart"
touch "$ROOT/dev/analysis.r" "$ROOT/dev/report.rmd"
touch "$ROOT/dev/Main.scala"
touch "$ROOT/dev/core.clj"
touch "$ROOT/dev/main.erl" "$ROOT/dev/main.hrl"
touch "$ROOT/dev/main.ml" "$ROOT/dev/main.mli"
touch "$ROOT/dev/boot.asm"
# web
touch "$ROOT/dev/index.html"
touch "$ROOT/dev/style.css"
touch "$ROOT/dev/style.scss" "$ROOT/dev/vars.sass"
touch "$ROOT/dev/style.less"
touch "$ROOT/dev/App.vue"
touch "$ROOT/dev/App.svelte"
touch "$ROOT/dev/module.wasm"
# data / config
touch "$ROOT/dev/config.json" "$ROOT/dev/tsconfig.jsonc"
touch "$ROOT/dev/config.yaml" "$ROOT/dev/docker-compose.yml"
touch "$ROOT/dev/Cargo.toml"
touch "$ROOT/dev/config.xml"
touch "$ROOT/dev/data.csv" "$ROOT/dev/data.tsv"
touch "$ROOT/dev/query.sql"
touch "$ROOT/dev/schema.graphql" "$ROOT/dev/schema.gql"
touch "$ROOT/dev/.env"
touch "$ROOT/dev/app.ini" "$ROOT/dev/app.cfg" "$ROOT/dev/app.conf"
touch "$ROOT/dev/.editorconfig"
# build / CI
touch "$ROOT/dev/Makefile" "$ROOT/dev/rules.mk"
touch "$ROOT/dev/CMakeLists.cmake"
touch "$ROOT/dev/Dockerfile"
touch "$ROOT/dev/build.gradle"
touch "$ROOT/dev/Cargo.lock"
touch "$ROOT/dev/default.nix"
# docs
touch "$ROOT/dev/README.md" "$ROOT/dev/page.mdx"
touch "$ROOT/dev/doc.rst"
touch "$ROOT/dev/report.tex" "$ROOT/dev/refs.bib"
touch "$ROOT/dev/notes.txt"
touch "$ROOT/dev/app.log"
touch "$ROOT/dev/fix.diff" "$ROOT/dev/patch.patch"
# compilés / artefacts
touch "$ROOT/dev/main.o" "$ROOT/dev/libfoo.a" "$ROOT/dev/libfoo.so"
touch "$ROOT/dev/Main.class"
touch "$ROOT/dev/module.pyc"
touch "$ROOT/dev/package.whl"
touch "$ROOT/dev/app.jar"

# ----------------------------------------------------------
# Permissions variées (Linux/Mac)
# ----------------------------------------------------------
mkdir -p "$ROOT/perms"
echo "readable" > "$ROOT/perms/read_only.txt"
chmod 444 "$ROOT/perms/read_only.txt"
echo "executable" > "$ROOT/perms/executable.sh"
chmod 755 "$ROOT/perms/executable.sh"
echo "private" > "$ROOT/perms/private.txt"
chmod 600 "$ROOT/perms/private.txt"

echo "Fixtures créées dans $ROOT :"
find "$ROOT" | sort
