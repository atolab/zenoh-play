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

import sys
import time
import argparse
import itertools
import json
import zenoh
import math
from zenoh import config

# --- Generating random sign --- --- --- --- --- ---
def sign():
    return 1 if random.random() < 0.5 else -1

# Using an ellipse to emulate SS-RSRP values
#
#  x^2/a^2 + y^2/b^2 = 1
#
# a = 9
# b = 3
# Focal points
# F1 = 6√2
# F2 = -6√2
#

A=9
B=3
F1 = (-6*math.sqrt(2), 0)
F2 = (6*math.sqrt(2), 0)

# --- Getting Y value --- --- --- --- --- ---
def get_y(x):
    return B*math.sqrt((1-(pow(x,2)/pow(A,2))))

# --- Computing distance --- --- --- --- --- ---
def distance(p1, p2):
    return math.sqrt( pow(p2[0] - p1[0],2) + pow(p2[1] - p1[1],2) )


def compute_rsrp(dist):
    return -10 * dist

def main():
    # --- Command line argument parsing --- --- --- --- --- ---
    parser = argparse.ArgumentParser(
        prog='z_sensor',
        description='zenoh pub example')
    parser.add_argument('--mode', '-m', dest='mode',
                        choices=['peer', 'client'],
                        type=str,
                        help='The zenoh session mode.')
    parser.add_argument('--peer', '-e', dest='peer',
                        metavar='LOCATOR',
                        action='append',
                        type=str,
                        help='Peer locators used to initiate the zenoh session.')
    parser.add_argument('--listener', '-l', dest='listener',
                        metavar='LOCATOR',
                        action='append',
                        type=str,
                        help='Locators to listen on.')
    parser.add_argument('--key', '-k', dest='key',
                        default='/paris/gnb/ss-rsrp',
                        type=str,
                        help='The key expression to publish onto.')
    parser.add_argument('--value', '-v', dest='value',
                        default='Pub from Python!',
                        type=str,
                        help='The value to publish.')
    parser.add_argument("--iter", dest="iter", type=int,
                        help="How many puts to perform")
    parser.add_argument('--config', '-c', dest='config',
                        metavar='FILE',
                        type=str,
                        help='A configuration file.')

    args = parser.parse_args()
    conf = zenoh.config_from_file(
        args.config) if args.config is not None else zenoh.Config()
    if args.mode is not None:
        conf.insert_json5("mode", json.dumps(args.mode))
    if args.peer is not None:
        conf.insert_json5("peers", json.dumps(args.peer))
    if args.listener is not None:
        conf.insert_json5("listeners", json.dumps(args.listener))
    key = args.key
    value = args.value

    # zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

    # initiate logging
    zenoh.init_logger()

    print("Openning session...")
    session = zenoh.open(conf)

    print("Declaring key expression '{}'...".format(key), end='')
    rid = session.declare_expr(key)
    print(" => RId {}".format(rid))

    print("Declaring publication on '{}'...".format(rid))
    session.declare_publication(rid)

    id = int(value)
    while True:
        for x in range(-90, 90, 1):
            x = x/10
            y = get_y(x)
            point(x,y)
            dist =  distance(F1, point) if id%2 else  distance(F2, point)
            v = compute_rsrp(dist)

            print("Putting Data ('{}': '{}')...".format(rid, v))
            session.put(rid, v)

            time.sleep(0.5)


    session.undeclare_publication(rid)
    session.undeclare_expr(rid)
    session.close()



if __name__=='__main__':
    main()

