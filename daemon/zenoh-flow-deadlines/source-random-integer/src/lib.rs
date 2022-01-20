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
use rand::prelude::*;
use shared_types::DUsize;
use zenoh_flow::{zf_empty_state, Configuration, Context, Data, Node, Source, State, ZFResult};

#[derive(Debug)]
struct SourceRandomInteger;

impl Node for SourceRandomInteger {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        zf_empty_state!()
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

#[async_trait]
impl Source for SourceRandomInteger {
    async fn run(&self, _context: &mut Context, _state: &mut State) -> ZFResult<Data> {
        zenoh_flow::async_std::task::sleep(std::time::Duration::from_secs(5)).await;
        let mut rng = thread_rng();
        let number = rng.gen_range(1..100);
        println!("[SourceRandomInteger] Number generated: {}", number);
        Ok(Data::from::<DUsize>(DUsize::new(number)))
    }
}

zenoh_flow::export_source!(register);

fn register() -> ZFResult<Arc<dyn Source>> {
    Ok(Arc::new(SourceRandomInteger) as Arc<dyn Source>)
}
