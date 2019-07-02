#!/usr/bin/bash -ex
cd ..
env -i git reset --hard
export PATH=/usr/gnu/bin:$PATH:/opt/ooce/bin
make launch
