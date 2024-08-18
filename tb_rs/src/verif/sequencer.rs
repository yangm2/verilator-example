//!
//! Associate function(s), Structs+methods for a testbench Sequencer
//!

use super::{Objectify, PhasingB};
use crate::verif::{PhasingA};
use std::collections::HashMap;

pub struct Sequencer {
    // private fields prevent direct construction by external code
    name: String,
    component_db: HashMap<String, Box<dyn PhasingA>>,
    pub seq_item_export: Box<dyn Fn() -> u32>,
    // rsp_export: verif::TlmPort<u32>,
}

pub fn new(name: &str) -> Sequencer {
    Sequencer {
        name: name.to_string(),
        component_db: HashMap::new(),
        seq_item_export: Box::new(|| 3),
        // rsp_export: verif::TlmPort { it: 0 },
    }
}

impl Objectify for Sequencer {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl PhasingB for Sequencer {
    fn connect<T: PhasingB>(&self, other: T) {
        todo!()
    }

    fn elaborate(&self) {
        todo!()
    }

    fn run(&self) {
        todo!()
    }

    fn shutdown(&self) {
        todo!()
    }
}
