#!/usr/bin/env bash
# Benchmark ruls vs eza sur des fixtures volumineuses
# Usage: ./benchmark.sh

set -e

RULS="./target/release/ruls"
BENCH_ROOT="bench_fixtures"
WARMUP=10
RUNS=500

# ----------------------------------------------------------
# Vérifications
# ----------------------------------------------------------
if ! command -v hyperfine &>/dev/null; then
    echo "Erreur : hyperfine n'est pas installé." >&2
    exit 1
fi
if ! command -v eza &>/dev/null; then
    echo "Erreur : eza n'est pas installé." >&2
    exit 1
fi
if [ ! -f "$RULS" ]; then
    echo "Binary ruls introuvable, build en cours..."
    cargo build --release --quiet
fi

# ----------------------------------------------------------
# Génération des fixtures
# ----------------------------------------------------------
echo "Génération des fixtures dans $BENCH_ROOT..."
rm -rf "$BENCH_ROOT"
mkdir -p "$BENCH_ROOT"

# Répertoire plat — beaucoup de fichiers de types variés
mkdir -p "$BENCH_ROOT/flat"
EXTENSIONS=(rs py js ts go c cpp h java rb sh md txt json yaml toml xml csv sql log ini cfg)
for i in $(seq 1 500); do
    ext=${EXTENSIONS[$((i % ${#EXTENSIONS[@]}))]}
    echo "content $i" > "$BENCH_ROOT/flat/file_$(printf '%04d' $i).$ext"
done

# Répertoire profond — arborescence imbriquée
mkdir -p "$BENCH_ROOT/deep"
for d in $(seq 1 20); do
    dir="$BENCH_ROOT/deep/dir_$(printf '%02d' $d)"
    mkdir -p "$dir"
    for i in $(seq 1 50); do
        echo "content $i" > "$dir/file_$(printf '%02d' $i).txt"
    done
done

# Répertoire mixte — fichiers + dossiers + symlinks
mkdir -p "$BENCH_ROOT/mixed"
for i in $(seq 1 100); do
    echo "content $i" > "$BENCH_ROOT/mixed/file_$i.rs"
done
for i in $(seq 1 20); do
    mkdir -p "$BENCH_ROOT/mixed/dir_$i"
done
for i in $(seq 1 10); do
    ln -sf "$BENCH_ROOT/mixed/file_$i.rs" "$BENCH_ROOT/mixed/link_$i"
done

echo "Fixtures prêtes."
echo "  flat/   : $(ls $BENCH_ROOT/flat | wc -l) fichiers"
echo "  deep/   : $(find $BENCH_ROOT/deep -type f | wc -l) fichiers dans $(find $BENCH_ROOT/deep -type d | wc -l) dossiers"
echo "  mixed/  : $(ls $BENCH_ROOT/mixed | wc -l) entrées"
echo ""

# ----------------------------------------------------------
# Benchmarks
# ----------------------------------------------------------

echo "════════════════════════════════════════"
echo " 1. Répertoire plat — 500 fichiers"
echo "════════════════════════════════════════"
hyperfine --warmup $WARMUP --runs $RUNS \
    "$RULS $BENCH_ROOT/flat > /dev/null" \
    "eza $BENCH_ROOT/flat > /dev/null" \
    --command-name "ruls" --command-name "eza" \
    --shell bash

echo ""
echo "════════════════════════════════════════"
echo " 2. Répertoire plat — mode long"
echo "════════════════════════════════════════"
hyperfine --warmup $WARMUP --runs $RUNS \
    "$RULS -l $BENCH_ROOT/flat > /dev/null" \
    "eza -l $BENCH_ROOT/flat > /dev/null" \
    --command-name "ruls -l" --command-name "eza -l" \
    --shell bash

echo ""
echo "════════════════════════════════════════"
echo " 3. Plusieurs répertoires en argument"
echo "════════════════════════════════════════"
hyperfine --warmup $WARMUP --runs $RUNS \
    "$RULS $BENCH_ROOT/flat $BENCH_ROOT/mixed > /dev/null" \
    "eza $BENCH_ROOT/flat $BENCH_ROOT/mixed > /dev/null" \
    --command-name "ruls multi" --command-name "eza multi" \
    --shell bash

echo ""
echo "════════════════════════════════════════"
echo " 4. Répertoire mixte — fichiers + dossiers + symlinks"
echo "════════════════════════════════════════"
hyperfine --warmup $WARMUP --runs $RUNS \
    "$RULS -l $BENCH_ROOT/mixed > /dev/null" \
    "eza -l $BENCH_ROOT/mixed > /dev/null" \
    --command-name "ruls -l mixed" --command-name "eza -l mixed" \
    --shell bash

# ----------------------------------------------------------
# Nettoyage
# ----------------------------------------------------------
echo ""
echo "Nettoyage des fixtures..."
rm -rf "$BENCH_ROOT"
echo "Terminé."
