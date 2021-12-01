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
use async_trait::async_trait;
use shared_types::DUsize;
use zenoh_flow::runtime::message::DataMessage;
use zenoh_flow::{zf_empty_state, Configuration, Context, Node, Sink, State, ZFResult};

struct SinkDisplay;

impl Node for SinkDisplay {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _dyn_state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

#[async_trait]
impl Sink for SinkDisplay {
    async fn run(
        &self,
        _context: &mut Context,
        _dyn_state: &mut State,
        mut input: DataMessage,
    ) -> ZFResult<()> {
        let number = input.get_inner_data().try_get::<DUsize>()?;
        println!("[Sink-Display] Received value: {}", number.value());

        Ok(())
    }
}

zenoh_flow::export_sink!(register);

fn register() -> ZFResult<Arc<dyn Sink>> {
    Ok(Arc::new(SinkDisplay) as Arc<dyn Sink>)
}
