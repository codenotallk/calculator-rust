#!/bin/bash

ab -n 1000 -c 100 'http://localhost:1234/v1/calculate?operation=sum&value_1=10&value_2=10'