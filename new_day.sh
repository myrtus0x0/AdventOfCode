#!/bin/bash

if [[ $1 =~ [0-9]{1,2} ]]; then
    mkdir -p $1
    echo "Directory for day $1 created!"
    echo "Changing to the newly created directory..."
    cd ./$1/
else 
    echo "Directory not created, invalid input!"
fi