#!/bin/sh -l
matrix=$(doc-status "$@")
echo "matrix<<EOF" >> $GITHUB_OUTPUT
echo $matrix >> $GITHUB_OUTPUT
echo "EOF" >> $GITHUB_OUTPUT
