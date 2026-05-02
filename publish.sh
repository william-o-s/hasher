#!/bin/bash
# Run `chmod +x publish.sh` once to make it executable.

# 1. Get the version passed as an argument (e.g., 0.1.0)
VERSION=$1

if [ -z "$VERSION" ]; then
    echo "❌ Error: No version provided."
    echo "Usage: ./publish.sh 0.1.0"
    exit 1
fi

# 2. Extract version from Cargo.toml
# This looks for the first instance of 'version = "X.X.X"'
# This will be compared to the GitHub release version for consistency
CARGO_VERSION=$(grep -m 1 '^version = ' Cargo.toml | sed 's/version = "//;s/"//')

# 3. Compare them
if [ "$VERSION" != "$CARGO_VERSION" ]; then
    echo "❌ Version Mismatch!"
    echo "Cargo.toml says: $CARGO_VERSION"
    echo "You provided:    $VERSION"
    echo "Please update Cargo.toml first."
    exit 1
fi

# 4. Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

# 5. Confirm and Push
read -p "Pushing v$VERSION to GitHub. Proceed? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Ensure we are on main and up to date
    echo "🚀 Pulling changes from main..."
    git checkout main
    git pull origin main
    # Tag and Push
    echo "🚀 Tagging and pushing..."
    git tag -a "v$VERSION" -m "Release v$VERSION"
    git push origin main
    git push origin "v$VERSION"
    echo "🚀 Tag v$VERSION pushed! Check GitHub Actions for build status."
else
    echo "Aborted."
fi
