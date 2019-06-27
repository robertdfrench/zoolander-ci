#!/usr/bin/bash -ex
BASEDIR=/export
ls -RAhlF $HOME
useradd -G staff -b $BASEDIR -m cirrus
passwd -N cirrus
mkdir -p $BASEDIR/cirrus/.ssh
cp $HOME/.ssh/authorized_keys $BASEDIR/cirrus/.ssh
chown -R cirrus $BASEDIR/cirrus/.ssh

pkg install git vim build-essential zip
