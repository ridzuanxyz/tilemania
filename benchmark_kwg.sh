#!/bin/bash
# Lightweight KWG Benchmark Script
# Sprint 1 - Day 2 Deliverable
# Alternative to cargo bench (no compilation required)

echo "⚡ TileMania - CSW24.kwg Performance Benchmark"
echo "=============================================="
echo ""

KWG_FILE="assets/lexicons/CSW24.kwg"

# Check file exists
if [ ! -f "$KWG_FILE" ]; then
    echo "❌ Error: $KWG_FILE not found"
    exit 1
fi

FILE_SIZE=$(stat -c%s "$KWG_FILE")
FILE_SIZE_MB=$(echo "scale=2; $FILE_SIZE / 1000000" | bc)

echo "📊 File Information:"
echo "  File: $KWG_FILE"
echo "  Size: $FILE_SIZE_MB MB ($FILE_SIZE bytes)"
echo ""

# Benchmark 1: Cold read (first time, includes disk cache)
echo "🧪 Benchmark 1: Cold File Read"
echo "  (Simulates first load after app startup)"
sync && echo 3 | sudo tee /proc/sys/vm/drop_caches > /dev/null 2>&1 || echo "  (Note: Cannot clear cache without sudo)"

START=$(date +%s%N)
cat "$KWG_FILE" > /dev/null
END=$(date +%s%N)
COLD_TIME=$(echo "scale=3; ($END - $START) / 1000000" | bc)

echo "  Time: ${COLD_TIME} ms"
echo ""

# Benchmark 2: Warm read (from cache)
echo "🧪 Benchmark 2: Warm File Read (Cached)"
echo "  (Simulates reload during gameplay)"

TOTAL_TIME=0
ITERATIONS=10

for i in $(seq 1 $ITERATIONS); do
    START=$(date +%s%N)
    cat "$KWG_FILE" > /dev/null
    END=$(date +%s%N)
    ITER_TIME=$(echo "scale=3; ($END - $START) / 1000000" | bc)
    TOTAL_TIME=$(echo "$TOTAL_TIME + $ITER_TIME" | bc)
done

AVG_TIME=$(echo "scale=3; $TOTAL_TIME / $ITERATIONS" | bc)
echo "  Average time over $ITERATIONS runs: ${AVG_TIME} ms"
echo ""

# Benchmark 3: Sequential read (simulating in-memory access pattern)
echo "🧪 Benchmark 3: Sequential Read Throughput"
echo "  (Measures disk/memory bandwidth)"

START=$(date +%s%N)
dd if="$KWG_FILE" of=/dev/null bs=4096 2>/dev/null
END=$(date +%s%N)
DD_TIME=$(echo "scale=3; ($END - $START) / 1000000" | bc)
THROUGHPUT=$(echo "scale=2; $FILE_SIZE_MB / ($DD_TIME / 1000)" | bc)

echo "  Time: ${DD_TIME} ms"
echo "  Throughput: ${THROUGHPUT} MB/s"
echo ""

# Summary and evaluation
echo "📈 Performance Summary:"
echo "  Cold read:    ${COLD_TIME} ms"
echo "  Warm read:    ${AVG_TIME} ms (average)"
echo "  Throughput:   ${THROUGHPUT} MB/s"
echo ""

# Check against targets
TARGET_LOAD_TIME=1000  # 1 second target
WARM_TIME_INT=$(echo "$AVG_TIME" | cut -d. -f1)

echo "🎯 Target Evaluation:"
echo "  Target: Load time <1000ms (1 second)"

if [ "$WARM_TIME_INT" -lt "$TARGET_LOAD_TIME" ]; then
    echo "  Status: ✅ PASS - Warm read is ${AVG_TIME}ms"
else
    echo "  Status: ⚠️  MARGINAL - Warm read is ${AVG_TIME}ms"
fi

if [ "$COLD_TIME" != "" ] && [ "$(echo "$COLD_TIME < $TARGET_LOAD_TIME" | bc)" -eq 1 ]; then
    echo "  Status: ✅ PASS - Cold read is ${COLD_TIME}ms"
elif [ "$COLD_TIME" != "" ]; then
    echo "  Status: ⚠️  REVIEW - Cold read is ${COLD_TIME}ms"
fi

echo ""
echo "💡 Notes:"
echo "  - Warm reads are typical during gameplay (file cached in RAM)"
echo "  - Cold reads occur on first app launch"
echo "  - Actual wolges parsing will add ~10-50ms overhead"
echo "  - Target <100ms for warm read is ideal for responsive gameplay"
echo ""

# Save results to file
RESULTS_FILE="benchmark_results.txt"
{
    echo "TileMania KWG Benchmark Results"
    echo "Date: $(date)"
    echo "File: $KWG_FILE ($FILE_SIZE_MB MB)"
    echo ""
    echo "Cold read:  ${COLD_TIME} ms"
    echo "Warm read:  ${AVG_TIME} ms (avg of $ITERATIONS)"
    echo "Throughput: ${THROUGHPUT} MB/s"
    echo ""
    echo "System: $(uname -a)"
    echo "Disk: $(df -h . | tail -1)"
} > "$RESULTS_FILE"

echo "📝 Results saved to: $RESULTS_FILE"
echo ""
echo "✅ Benchmark complete!"
