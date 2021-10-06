#!/bin/bash

./razor-rpc-server &
./ztool create -n dpool -m raidz
