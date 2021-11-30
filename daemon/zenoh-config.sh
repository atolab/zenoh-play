#!/bin/bash

curl -X PUT -H 'content-type:application/properties' -d "url=http://localhost:8086" http://localhost:8000/@/router/local/plugin/storages/backend/influxdb
curl -X PUT -H 'content-type:application/properties' -d "path_expr=/daemon/**;db=daemon;create_db" http://localhost:8000/@/router/local/plugin/storages/backend/influxdb/storage/daemon
