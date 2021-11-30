# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>

import argparse
import json
import random
import sys
import time
import zenoh
from zenoh import Zenoh, Value

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_sensor',
    description='zenoh put example')
parser.add_argument('--mode', '-m', dest='mode',
                    choices=['peer', 'client'],
                    type=str,
                    help='The zenoh session mode.')
parser.add_argument('--peer', '-e', dest='peer',
                    metavar='LOCATOR',
                    action='append',
                    type=str,
                    help='Peer locators used to initiate the zenoh session.')
parser.add_argument('--id', '-i', dest='id',
                    default=1,
                    type=int,
                    help='The id of the sensor.')

args = parser.parse_args()
conf = {}
if args.mode is not None:
    conf["mode"] = args.mode
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)

assert args.id > 0
path = '/daemon/sensor/' + str(args.id)

# initiate logging
zenoh.init_logger()

print("Openning session...")
z = Zenoh(conf)

print("New workspace...")
workspace = z.workspace()

while True:
    value = args.id + random.random()
    print("Put Data ('{}': '{}')...".format(path, value))
    workspace.put(path, value)
    time.sleep(args.id)

z.close()
