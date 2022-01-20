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
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::{
    default_input_rule, default_output_rule, types::ZFResult, Configuration, Context, Data,
    DataMessage, LocalDeadlineMiss, Node, NodeOutput, Operator, PortId, State, Token, ZFError,
};

static INPUT: &str = "In";
static OUTPUT: &str = "Out";

#[derive(ZFState, Clone, Debug)]
struct AccumulatorState {
    value: usize,
}

impl AccumulatorState {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

#[derive(Debug)]
struct OperatorAccumulator;

impl Node for OperatorAccumulator {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        Ok(State::from(AccumulatorState::new()))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

impl Operator for OperatorAccumulator {
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
        dyn_state: &mut State,
        inputs: &mut HashMap<PortId, DataMessage>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        // Get INPUT.
        let mut input = inputs
            .remove(INPUT)
            .ok_or_else(|| ZFError::InvalidData("No data".into()))?;
        let number = input.get_inner_data().try_get::<DUsize>()?;

        // Accumulates.
        let mut state = dyn_state.try_get::<AccumulatorState>()?;
        state.value += number.value();

        let mut results = HashMap::with_capacity(1);
        results.insert(
            OUTPUT.into(),
            Data::from::<DUsize>(DUsize::new(state.value)),
        );

        // Sleep.
        let mut rng = thread_rng();
        let sleep_time = rng.gen_range(25..=75);
        println!("[OperatorAccumulator] Sleeping for {}ms", sleep_time);
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
    Ok(Arc::new(OperatorAccumulator) as Arc<dyn Operator>)
}
