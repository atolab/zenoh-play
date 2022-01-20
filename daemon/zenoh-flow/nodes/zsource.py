##
# Copyright (c) 2017, 2021 ADLINK Technology Inc.
##
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
##
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
##
# Contributors:
# ADLINK zenoh team, <zenoh@adlink-labs.tech>
##

from zenoh_flow import Inputs, Outputs, Source
import zenoh
from zenoh import Zenoh
import struct

value = 0
has_value = False
zenoh.init_logger()


def zlistener(change):
    global value, has_value
    value = change.value.get_content()
    has_value = True


class ZSourceState:
    def __init__(self, configuration={}):
        self.key_expr = '/daemon/sensor/*'
        if configuration is not None and configuration.get('key-expr') is not None:
            self.key_expr = configuration['key-expr']

        conf = {
            'mode': 'peer'
        }
        self.zenoh = Zenoh(conf)
        self.ws = self.zenoh.workspace()
        self.sub = self.ws.subscribe(self.key_expr, zlistener)

    def close(self):
        self.sub.close()
        self.zenoh.close()


class ZSource(Source):
    def initialize(self, configuration):
        return ZSourceState(configuration)

    def finalize(self, state):
        state.close()
        return None

    def run(self, _ctx, state):
        global value, has_value
        while (has_value == False):
            pass
        has_value = False
        ba = bytearray(struct.pack("f", value))

        return ba


def register():
    return ZSource
