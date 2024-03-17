#!/bin/sh -l
matrix=$(doc-status "$1")
echo "matrix<<EOF" >> $GITHUB_OUTPUT
echo $matrix >> $GITHUB_OUTPUT
echo "EOF" >> $GITHUB_OUTPUT
