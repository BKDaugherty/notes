#!/usr/bin/env bash

GIT_DIR=$(git rev-parse --git-dir)

echo "Installing hooks..."
# I don't really know why it only works if this is an actual file.
# And since this is a hobby project only I will likely work on, I don't really care :P
rm $GIT_DIR/hooks/pre-commit
cp ../scripts/pre-commit.bash $GIT_DIR/hooks/pre-commit
echo "Done!"
