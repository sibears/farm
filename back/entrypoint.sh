#!/bin/bash
./wait-for-it.sh postgres:5432 -q
./sibears_farm