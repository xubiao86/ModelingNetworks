#!/bin/bash

current_dir=$PWD
echo "Current directory is: $current_dir"
cd $current_dir

/usr/local/bin/python3.11 $current_dir//request_gen.py