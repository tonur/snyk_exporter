#!/usr/bin/env bash

docker run --rm -v ${PWD}:/local --user $UID:$GID openapitools/openapi-generator-cli generate -i "https://api.snyk.io/rest/openapi/2024-01-23" -g rust -o /local/openapi_generator/rust