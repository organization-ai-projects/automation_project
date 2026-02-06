#!/bin/bash
# Check that stable products do not depend on unstable products
# This enforces Rule 2: No contamination from unstable to stable

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
STABLE_DIR="$REPO_ROOT/projects/products/stable"
UNSTABLE_DIR="$REPO_ROOT/projects/products/unstable"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "Checking stable → unstable dependencies..."
echo "Stable products: $STABLE_DIR"
echo "Unstable products: $UNSTABLE_DIR"
echo ""

# Find all Cargo.toml files in stable products
stable_tomls=$(find "$STABLE_DIR" -name "Cargo.toml" -type f)

violations_found=0

for toml in $stable_tomls; do
    # Get the product name for reporting
    product_name=$(dirname "$toml" | sed "s|$STABLE_DIR/||")
    
    # Check if this Cargo.toml references any unstable products
    # Look for path dependencies that point to unstable/
    if grep -q "path.*unstable" "$toml" 2>/dev/null; then
        echo -e "${RED}❌ VIOLATION:${NC} Stable product depends on unstable"
        echo "   Product: $product_name"
        echo "   File: $toml"
        echo "   Dependencies:"
        grep "path.*unstable" "$toml" | sed 's/^/     /'
        echo ""
        violations_found=$((violations_found + 1))
    fi
done

# Also check the root Cargo.toml workspace dependencies
echo "Checking workspace dependencies in root Cargo.toml..."
if [ -f "$REPO_ROOT/Cargo.toml" ]; then
    # Check for any unstable paths in workspace dependencies
    if grep -q "path.*products/unstable" "$REPO_ROOT/Cargo.toml" 2>/dev/null; then
        echo -e "${YELLOW}⚠️  WARNING:${NC} Workspace includes unstable products"
        echo "   This is allowed, but verify stable products don't reference them"
        echo ""
    fi
fi

# Summary
echo "=========================================="
if [ $violations_found -eq 0 ]; then
    echo -e "${GREEN}✓ PASSED:${NC} No stable → unstable dependencies found"
    echo "All stable products properly isolated from unstable code."
    exit 0
else
    echo -e "${RED}✗ FAILED:${NC} Found $violations_found violation(s)"
    echo ""
    echo "Stable products MUST NOT depend on unstable products."
    echo "To fix:"
    echo "  1. Remove the unstable dependency from the stable product"
    echo "  2. OR promote the unstable product to stable first"
    echo "  3. OR extract needed functionality into a stable library"
    echo ""
    echo "See projects/products/README.md for details on Rule 2."
    exit 1
fi
