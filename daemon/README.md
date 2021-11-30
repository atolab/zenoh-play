# Influxdb playground

NOTE: the following steps have been tested on Ubuntu 20.04 64 bits

## 0. Install Rust ecosystem

$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install cargo-deb

## 1. Clone and build zenoh and zenoh-plugin-storages 

$ cd
$ git clone https://github.com/eclipse-zenoh/zenoh
$ cd zenoh
$ cargo build --release -p zenoh -p zenoh-plugin-storages -p zenoh-plugin-rest

On Ubuntu you can also generate and install the deb files:

$ cargo deb -p zenoh
$ cargo deb -p zenoh-plugin-storages
$ cargo deb -p zenoh-plugin-rest
$ apt install ./target/debian/*.deb

## 2. Install influxdb

$ sudo apt install influxdb

## 3. Clone and build the influxdb backend

$ git clone https://github.com/eclipse-zenoh/zenoh-backend-influxdb
$ cd zenoh-backend-influxdb
$ cargo build --release

On Ubuntu you can also generate and install the deb files:

$ cargo deb
$ apt install ./target/debian/*.deb

## 4. Start zenohd and configure the storages

$ curl -X PUT -H 'content-type:application/properties' -d "url=http:// localhost:8086" http://localhost:8000/@/router/local/plugin/storages/backend/influxdb
$ curl -X PUT -H 'content-type:application/properties' -d "path_expr=/daemon/**;db=daemon;create_db" http://localhost:8000/@/router/local/plugin/storages/backend/influxdb/storage/daemon

## 5. Clone and build zenoh-python

$ cd
$ git clone https://github.com/eclipse-zenoh/zenoh-python
$ cd zenoh-python
$ apt install python3-pip python3-launchpadlib
$ pip3 install -r requirements-dev.txt
$ python3 setup.py develop
