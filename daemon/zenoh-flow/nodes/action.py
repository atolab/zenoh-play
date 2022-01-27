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

from zenoh_flow import Sink
import zenoh
from zenoh import config


class ActionSinkState:
    def __init__(self, configuration={}):
        self.key_expr = '/daemon/action'
        if configuration is not None and configuration.get('key-expr') is not None:
            self.key_expr = configuration['key-expr']

        conf = zenoh.Config({
            'mode': 'peer'
        })
        self.zenoh = zenoh.open(conf)

    def close(self):
        self.zenoh.close()


class ActionSink(Sink):
    def initialize(self, configuration):
        return ActionSinkState(configuration)

    def finalize(self, _state):
        return None

    def run(self, _ctx, state, data):
        action = data.data.decode("utf-8")
        print(f'Action received {action}')
        state.zenoh.put(state.key_expr, action)


def register():
    return ActionSink
