#!/usr/bin/sh

docker run --network=host --env-file locust.env $1
