//
// Copyright (c) 2017, 2021 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//

use async_std::sync::Arc;
use rand::prelude::*;
use shared_types::DUsize;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use zenoh_flow::{
    default_input_rule, default_output_rule, types::ZFResult, zf_empty_state, Configuration,
    Context, Data, DataMessage, LocalDeadlineMiss, Node, NodeOutput, Operator, PortId, State,
    Token, ZFError,
};

static INPUT: &str = "In";
static OUTPUT: &str = "Out";

#[derive(Debug)]
struct OperatorTimesTwo;

impl Node for OperatorTimesTwo {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

impl Operator for OperatorTimesTwo {
    fn input_rule(
        &self,
        _context: &mut Context,
        state: &mut State,
        inputs: &mut HashMap<PortId, Token>,
    ) -> ZFResult<bool> {
        default_input_rule(state, inputs)
    }

    fn run(
        &self,
        _context: &mut Context,
        _state: &mut State,
        inputs: &mut HashMap<PortId, DataMessage>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        // Get INPUT.
        let mut input = inputs
            .remove(INPUT)
            .ok_or_else(|| ZFError::InvalidData("No data".into()))?;
        let number = input.get_inner_data().try_get::<DUsize>()?;

        // Multiply by 2.
        let mut results = HashMap::with_capacity(1);
        results.insert(
            OUTPUT.into(),
            Data::from::<DUsize>(DUsize::new(number.value() * 2)),
        );

        // Sleep.
        let mut rng = thread_rng();
        let sleep_time = rng.gen_range(50..=150);
        println!("[OperatorTimesTwo] Sleeping for {}ms", sleep_time);
        thread::sleep(Duration::from_millis(sleep_time));

        // Return new value.
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadline_miss: Option<LocalDeadlineMiss>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

zenoh_flow::export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(OperatorTimesTwo) as Arc<dyn Operator>)
}
