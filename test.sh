#!/bin/bash
cd tests
docker-compose up --build tester
docker-compose down
cd ../
