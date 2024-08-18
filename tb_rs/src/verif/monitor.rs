//!
//!  mod to encapsulate Monitor code
//!

use crate::verif::{self, Objectify, PhasingA};
use std::collections::HashMap;

use super::VirtualInterface;
/**
  Monitor
*/
pub struct Monitor {
    pub name: String,
    // heterogenous hashmap of Trait Objects
    pub component_db: HashMap<String, Box<dyn PhasingA>>,
    pub phase: verif::Phase,
    pub vif: verif::VirtualInterface,
}

impl<'a> Objectify for Monitor {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl<'a> PhasingA for Monitor {
    fn get_phase(&self) -> verif::Phase {
        dbg!("{}", &self.phase);
        println!("{}: {}", self.get_name(), &self.phase);
        self.phase.clone()
    }

    fn configure(&mut self) {
        // Top-Down configuration
        self.component_db.values_mut().for_each(|v| {
            v.configure();
        });
    }
}

pub fn new(name: &str) -> Monitor {
    Monitor {
        name: name.to_string(),
        component_db: HashMap::new(),
        phase: super::Phase::Allocated,
        vif: VirtualInterface {},
    }
}
