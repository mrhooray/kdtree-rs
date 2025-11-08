#!/usr/bin/env bash
set -euo pipefail

BASELINE_DIR="baseline"
TARGET_DIR="${CARGO_TARGET_DIR:-target}/criterion"
BASELINE_NAME="${CRITERION_BASELINE_NAME:-base}"
RUN_BASELINE="${CRITERION_RUN_NAME:-ci}"
THRESHOLD="${CRITERION_THRESHOLD:-0.08}"

if ! command -v cargo >/dev/null 2>&1; then
    echo "cargo not found in PATH" >&2
    exit 1
fi

if ! command -v critcmp >/dev/null 2>&1; then
    echo "critcmp is required for regression detection" >&2
    exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
    echo "jq is required for regression detection" >&2
    exit 1
fi

if [ ! -d "$BASELINE_DIR" ]; then
    echo "expected baseline directory at $BASELINE_DIR" >&2
    exit 1
fi

if [ "${CRITERION_SKIP_RUN:-0}" != "1" ]; then
    mkdir -p "$TARGET_DIR"
    rm -rf "${TARGET_DIR:?}/${RUN_BASELINE}"
    rsync -a "$BASELINE_DIR/" "$TARGET_DIR/"
    cargo bench --bench bench -- --save-baseline "$RUN_BASELINE"
else
    echo "skipping benchmark run; reusing existing data in $TARGET_DIR" >&2
fi

critcmp --target-dir "$(dirname "$TARGET_DIR")" "$BASELINE_NAME" "$RUN_BASELINE"

regressions=()
missing=()
threshold_pct=$(awk -v t="$THRESHOLD" 'BEGIN { printf "%.2f", t * 100 }')

echo
echo "compared to $BASELINE_NAME (median/mean):"

while IFS= read -r -d '' base_file; do
    bench_dir=$(dirname "$(dirname "$base_file")")
    bench_name=$(basename "$bench_dir")
    ci_file="$bench_dir/$RUN_BASELINE/estimates.json"

    if [ ! -f "$ci_file" ]; then
        missing+=("$bench_name")
        continue
    fi

    base_median=$(jq -er '.median.point_estimate' "$base_file" 2>/dev/null || echo "")
    ci_median=$(jq -er '.median.point_estimate' "$ci_file" 2>/dev/null || echo "")
    base_mean=$(jq -er '.mean.point_estimate' "$base_file" 2>/dev/null || echo "")
    ci_mean=$(jq -er '.mean.point_estimate' "$ci_file" 2>/dev/null || echo "")

    if [ -z "$base_median" ] || [ -z "$ci_median" ] || [ -z "$base_mean" ] || [ -z "$ci_mean" ] || [ "$base_median" = "null" ] || [ "$ci_median" = "null" ] || [ "$base_mean" = "null" ] || [ "$ci_mean" = "null" ]; then
        missing+=("$bench_name")
        continue
    fi

    delta_median=$(awk -v base="$base_median" -v current="$ci_median" 'BEGIN {
        if (base == 0) { print "0"; exit }
        printf "%.10f", (current - base) / base
    }')

    delta_mean=$(awk -v base="$base_mean" -v current="$ci_mean" 'BEGIN {
        if (base == 0) { print "0"; exit }
        printf "%.10f", (current - base) / base
    }')

    pct_median=$(awk -v d="$delta_median" 'BEGIN { printf "%.2f", d * 100 }')
    pct_mean=$(awk -v d="$delta_mean" 'BEGIN { printf "%.2f", d * 100 }')
    printf '  %-40s median=%+s%% mean=%+s%%\n' "$bench_name" "$pct_median" "$pct_mean"

    violated=0
    if awk -v delta="$delta_median" -v threshold="$THRESHOLD" 'BEGIN { exit !(delta > threshold) }'; then
        violated=1
    fi
    if awk -v delta="$delta_mean" -v threshold="$THRESHOLD" 'BEGIN { exit !(delta > threshold) }'; then
        violated=1
    fi

    if [ "$violated" -eq 1 ]; then
        regressions+=("$bench_name:median=${pct_median}%,mean=${pct_mean}%")
    fi
done < <(find "$TARGET_DIR" -path "*/base/estimates.json" -print0)

if [ ${#missing[@]} -gt 0 ]; then
    printf 'criterion regression scan missing change data for: %s\n' \
        "$(printf '%s ' "${missing[@]}" | sed 's/ $//')" >&2
fi

if [ ${#regressions[@]} -gt 0 ]; then
    echo
    echo "regressions detected (>${threshold_pct}%):"
    for entry in "${regressions[@]}"; do
        name=${entry%%:*}
        pct=${entry#*:}
        echo "  $name: $pct vs baseline"
    done
    exit 2
fi

echo
echo "no regressions detected (>${threshold_pct}%)."
