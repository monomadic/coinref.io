#!/bin/sh
git pull
service supervisor restart && service nginx restart
