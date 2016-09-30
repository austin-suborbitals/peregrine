#!/bin/bash

set -e

KCOV_INSTALL=$1
KCOV_BUILD=$2


if [ -f $KCOV_INSTALL/bin/kcov ]; then
    exit 0
else
    echo "kcov not found in: " $KCOV_INSTALL
fi

mkdir -p $KCOV_INSTALL
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
mkdir $KCOV_BUILD
pushd $KCOV_BUILD
cmake -DCMAKE_INSTALL_PREFIX=$KCOV_INSTALL ..
make
make install
rm -f master.tar.gz
rm -rf $(KCOV_BUILD)
popd
rm -rf kcov-master
