//!
//!  mod to encapsulate Monitor code
//!

use crate::verif::{self, Objectify, PhasingA};
use std::collections::HashMap;
/**
  Monitor
*/
pub struct Monitor {
    pub name: &'static str,
    // heterogenous hashmap of Trait Objects
    pub component_db: HashMap<String, Box<dyn PhasingA>>,
    pub phase: verif::Phase,
    pub vif: verif::VirtualInterface,
}

impl<'a> Objectify for Monitor {
    fn get_name(&self) -> String {
        self.name.to_string()
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
