#!/bin/sh
ssh -t root@172.104.179.250 "cd /www/coinref/ && bash scripts/deploy.sh"
