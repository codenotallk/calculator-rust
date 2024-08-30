#!/bin/bash

curl -X GET 'http://localhost:1234/v1/calculate?operation=sum&value_1=10&value_2=10'
echo
curl -X GET 'http://localhost:1234/v1/calculate?operation=subb&value_1=10&value_2=10'
echo
curl -X GET 'http://localhost:1234/v1/calculate?operation=multiply&value_1=10&value_2=10'
echo
curl -X GET 'http://localhost:1234/v1/calculate?operation=divide&value_1=10&value_2=10'
echo