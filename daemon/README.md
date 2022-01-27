# Daemon playground

This playground shows the step-by-step process of how zenoh pubs/subs, storages, and zenoh-flows can be independently added to a live zenoh system.
The main goal is to shows how zenoh and zenoh-flow can be used to support [H2020 DAEMON framework](https://h2020daemon.eu/), especially how InfluxDB backend can be used for storing real-time freshly published data and to provide historic data for off-line processing e.g. for machine learning.

Few slides showing the zenoh scenario of the playground below are available [here](https://drive.google.com/file/d/14xVkE4Q5_moI3Ps7GMbT1UamH-7w8Gy5/view?usp=sharing).

NOTE: the following steps have been tested on Ubuntu 20.04 64 bits.

## 0. Install Rust ecosystem
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo install cargo-deb
```

## 1. Clone and build zenoh and zenoh-plugin-storages

```sh
cd
git clone https://github.com/eclipse-zenoh/zenoh
cd zenoh
cargo build --release -p zenoh -p zenoh-plugin-storages -p zenoh-plugin-rest
```

On Ubuntu you can also generate and install the deb files:

```sh
cargo deb -p zenoh
cargo deb -p zenoh-plugin-storages
cargo deb -p zenoh-plugin-rest
sudo apt install ./target/debian/*.deb
```

## 2. Install influxdb

```sh
sudo apt install influxdb
```

## 3. Clone and build the influxdb backend

```sh
cd
git clone https://github.com/eclipse-zenoh/zenoh-backend-influxdb
cd zenoh-backend-influxdb
cargo build --release --all-targets
```

On Ubuntu you can also generate and install the deb files:

```sh
cargo deb
sudo apt install ./target/debian/*.deb
```

## 4. Start zenohd and configure the influxdb storages

In a first terminal:
```sh
RUST_LOG=debug zenohd
```

In a second terminal:
```sh
cd
cd zenoh-play/daemon
bash zenoh-config.sh
```

## 5. Clone and build zenoh-python

```sh
cd
git clone https://github.com/eclipse-zenoh/zenoh-python
cd zenoh-python
sudo apt install python3-pip python3-launchpadlib
pip3 install -r requirements-dev.txt
python3 setup.py develop
```

## 6. Run the pub-sub

Start a first publisher that push data every second.
In a first terminal:
```sh
cd
cd zenoh-play/daemon/src
python3 z_sensor.py -i 1
```

Start a second publisher that push data every two seconds.
In a second terminal:
```sh
cd
cd zenoh-play/daemon/src
python3 z_sensor.py -i 2
```

Start a subscriber to receive all data being published.
In a third terminal:
```sh
cd
cd zenoh-python/examples/zenoh
python3 z_sub.py -s '/daemon/**'
```

## 7. Retrive historical data

In a terminal, retrive the last minute data.
```sh
cd
cd zenoh-python/examples/zenoh
python3 z_get.py -s '/daemon/**?(starttime=now()-1m;stoptime=now())'
```

## 8. Clone and build Zenoh-Flow runtime example

```sh
cd
git clone https://github.com/atolab/zenoh-flow-examples
cd zenoh-flow-examples
cargo build --release -p runtime
```

## 9. Clone and build zenoh-flow-python, and python wrappers

```sh
cd
git clone https://github.com/atolab/zenoh-flow-python/
cd zenoh-flow-python/zenoh-flow-python
pip3 install -r requirements-dev.txt
python3 setup.py bdist_wheel
pip3 install dist/eclipse_zenoh_flow-0.1.0-cp36-abi3-linux_x86_64.whl
cd ..
cargo build --release -p py-op -p py-sink -p py-source
```

## 10. Run Zenoh Flow

```sh
cd
~/zenoh-flow-examples/target/release/runtime -r daemon -g ~/zenoh-play/daemon/zenoh-flow/dataflow.yml -l ~/zenoh-play/daemon/zenoh-flow/loader-config.yml
```
