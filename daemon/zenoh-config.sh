#!/bin/bash

ZENOH_URL="http://localhost:8000"
INFLUXDB_URL="http://localhost:8086"
NAME="daemon"
PATH="/${NAME}/**"

curl -X PUT -H 'content-type:application/properties' -d "url=${URL}" ${ZENOH_URL}/@/router/local/plugin/storages/backend/influxdb
curl -X PUT -H 'content-type:application/properties' -d "path_expr=${PATH};db=${NAME};create_db" ${ZENOH_URL}/@/router/local/plugin/storages/backend/influxdb/storage/${NAME}
