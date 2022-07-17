#!/bin/bash

if [ $# -ne 2 ]
then
  echo "Usage: sh $0 filename key"
  exit 1
fi

filename=$1
key=$2
cat $filename | awk -v v1=$key '$0 ~ v1 {print $0}'
