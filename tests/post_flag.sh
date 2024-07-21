#!/bin/bash

curl -s -H 'Authorization: sibears1cool' -X POST -d "[\"$1\"]" http://localhost:8000/api/post_simple
