#!/bin/bash

if [ $# -ne 3 ]
then
    echo "Usage:$0 filename key replaced"
    exit 1
fi

filename=$1
ori_key=$2
des_key=$3

cat $filename | sed "s/$ori_key/$des_key/g" > $filename".tmp"
