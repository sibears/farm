#!/bin/bash
./wait-for-it.sh postgres:5432 -q
diesel setup
diesel migration run
./sibears_farm