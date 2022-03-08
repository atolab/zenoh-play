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

from zenoh_flow import Inputs, Operator, Outputs
import struct

class ComputeOp(Operator):
    def initialize(self, _configuration):
         return None

    def finalize(self, _state):
        return None

    def input_rule(self, _ctx, _state, _tokens):
        return True

    def output_rule(self, _ctx, _state, outputs, _deadline_miss):
        return outputs

    def run(self, _ctx, _state, inputs):
        # Getting the inputs
        data = inputs.get('Value').data

        value = struct.unpack('f', data)[0]

        outputs = {}

        if value < -90:
            outputs['Action'] = b'handover'

        outputs['Value'] = data
        return outputs


def register():
    return ComputeOp