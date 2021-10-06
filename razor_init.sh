#!/bin/bash

./razor-rpc-server &
./ztool create dpool raidz --ashift=12 --mountpoint=none
