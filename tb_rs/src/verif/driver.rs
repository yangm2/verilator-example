/**
  Driver
*/
use super::{Objectify, VirtualInterface};
use crate::verif::{self, PhasingA};
use std::collections::HashMap;

pub struct Driver {
    name: String,
    // heterogenous hashmap of Trait Objects
    pub component_db: HashMap<String, &'static mut dyn PhasingA>,
    pub phase: verif::Phase,
    pub vif: verif::VirtualInterface,
    pub seq_item_port: verif::tlm::TlmPort<u32>,
    pub rsp_port: verif::tlm::TlmPort<u32>,
}

impl Objectify for Driver {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl PhasingA for Driver {
    fn get_phase(&self) -> verif::Phase {
        dbg!("{}", &self.phase);
        println!("{}: {}", self.get_name(), &self.phase);
        self.phase.clone()
    }

    fn configure(&mut self) {
        // Top-Down configuration
        for v in self.component_db.values_mut() {
            v.configure();
        }
    }
}

pub fn new(name: &str) -> Driver {
    Driver {
        name: name.to_string(),
        component_db: HashMap::new(),
        phase: verif::Phase::Allocated,
        vif: VirtualInterface {},
        seq_item_port: super::tlm::new(3),
        rsp_port: super::tlm::new(3),
    }
}
