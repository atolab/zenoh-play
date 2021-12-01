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
use rand::seq::SliceRandom;
use rand::thread_rng;
use shared_types::DUsize;
use std::collections::HashMap;
use zenoh_flow::{
    default_input_rule, default_output_rule, zf_empty_state, Configuration, Context, Data,
    DataMessage, LocalDeadlineMiss, Node, NodeOutput, Operator, PortId, State, Token, ZFError,
    ZFResult,
};

static INPUT_PLUS_ONE: &str = "In-Plus1";
static INPUT_TIMES_TWO: &str = "In-Times2";
static INPUT_MINUS_ONE: &str = "In-Minus1";
static INPUT_ACCUMULATOR: &str = "In-Accumulator";
static OUTPUT: &str = "Out";

#[derive(Debug)]
struct OperatorSelection;

impl Node for OperatorSelection {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

impl Operator for OperatorSelection {
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
        let mut data_messages = Vec::with_capacity(3);
        data_messages.push(
            inputs
                .remove(INPUT_PLUS_ONE)
                .ok_or_else(|| ZFError::InvalidData("In-Plus1 missing".into()))?,
        );

        data_messages.push(
            inputs
                .remove(INPUT_TIMES_TWO)
                .ok_or_else(|| ZFError::InvalidData("In-Times2 missing".into()))?,
        );

        data_messages.push(
            inputs
                .remove(INPUT_MINUS_ONE)
                .ok_or_else(|| ZFError::InvalidData("In-Minus1 missing".into()))?,
        );

        data_messages.push(
            inputs
                .remove(INPUT_ACCUMULATOR)
                .ok_or_else(|| ZFError::InvalidData("In-Accumulator missing".into()))?,
        );

        let mut possible_values = Vec::with_capacity(3);
        data_messages.iter_mut().for_each(|data_message| {
            if data_message.get_missed_end_to_end_deadlines().is_empty() {
                let number = data_message.get_inner_data().try_get::<DUsize>().unwrap();
                possible_values.push(number.value());
            }
        });

        let mut rng = thread_rng();
        let choice: usize = match possible_values.choose(&mut rng) {
            Some(value) => *value,
            None => 0,
        };

        let mut results = HashMap::with_capacity(1);
        results.insert(OUTPUT.into(), Data::from::<DUsize>(DUsize::new(choice)));

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
    Ok(Arc::new(OperatorSelection) as Arc<dyn Operator>)
}
