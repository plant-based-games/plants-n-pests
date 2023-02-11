#!/usr/bin/env sh

docker run --rm \
    --network=host \
    --env-file .env \
    plantsnpests
