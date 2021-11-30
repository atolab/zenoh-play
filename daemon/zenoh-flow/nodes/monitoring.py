##
## Copyright (c) 2017, 2021 ADLINK Technology Inc.
##
## This program and the accompanying materials are made available under the
## terms of the Eclipse Public License 2.0 which is available at
## http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
## which is available at https://www.apache.org/licenses/LICENSE-2.0.
##
## SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
##
## Contributors:
##   ADLINK zenoh team, <zenoh@adlink-labs.tech>
##

from zenoh_flow import Sink
import zenoh
from zenoh import Zenoh, Value

class MonitoringSinkState:
    def __init__(self, configuration):
        self.key_expr = '/daemon/monitoring'
        if configuration['key-expr'] is not None:
             self.key_expr = configuration['key-expr']

        conf = {
            'mode':'peer'
        }
        self.zenoh = Zenoh(conf)
        self.ws = self.zenoh.workspace()

    def close(self):
        self.zenoh.close()


class MonitoringSink(Sink):
    def initialize(self, configuration):
        return MonitoringSinkState(configuration)

    def finalize(self, _state):
        return None

    def run(self, _ctx, state, data):
        monitoring = value = struct.unpack('f', data.data)
        print(f'Monitoring received {monitoring}')
        state.workspace.put(state.key_expr, monitoring)

def register():
    return MonitoringSink
