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

## 6. Clone and build Zenoh-Flow runtime example

$ cd
$ git clone https://github.com/atolab/zenoh-flow-examples
$ cd zenoh-flow-examples
$ cargo build --release -p runtime

## 7. Clone and build zenoh-flow-python, and python wrappers

$ cd
$ git clone https://github.com/atolab/zenoh-flow-python/
$ cd zenoh-flow-python/zenoh-flow-python
$ pip3 install -r requirements-dev.txt
$ python3 setup.py bdist_wheel
$ pip3 install dist/eclipse_zenoh_flow-0.1.0-cp36-abi3-linux_x86_64.whl
$ cd ..
$ cargo build --release -p py-op -p py-sink

## 8. Run it!

*TODO*