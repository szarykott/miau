#!/bin/bash

mmdc=node_modules/.bin/mmdc 
if [ ! -f $mmdc ]; then 
    npm install
fi

$mmdc -i assets/basicflow.mmdc -o assets/basicflow.png -t neutral \
    -w 1000 -H 800
