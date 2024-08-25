#!/bin/sh
if [[ $# != 2 ]]; then
  echo "[usage] ./create_dummy_headers.sh src_dir dst_dir"
  exit 1
fi

srcdir=${1}
dstdir=${2}
if [[ !(-d ${srcdir}) ]]; then
  echo "src_dir `${srcdir}` not found."
  exit 1
fi
if [[ !(-d ${dstdir}) ]]; then
  mkdir -p ${dstdir}
fi

files=`find ${srcdir} -type f -printf '%P\n'`
for file in ${files}; do
  dstpath=${dstdir}/${file}
  install -D /dev/null ${dstpath}
  echo "#pragma INCLUDE<${file}>" > ${dstpath}
done


