#!/bin//bash

set -eo pipefail

echo "Building amd64 image..."
docker build --platform linux/amd64 --tag bayram/dcron:v0.1.0-amd64 .
docker push bayram/dcron:v0.1.0-amd64

echo "Building arm64 image..."
docker build --platform linux/arm64 --tag bayram/dcron:v0.1.0-arm64 .
docker push bayram/dcron:v0.1.0-arm64
