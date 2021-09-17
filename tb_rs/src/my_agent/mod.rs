use crate::verif::{self, Objectify, PhasingA, PhasingB, TlmPort, VirtualInterface};
use std::collections::HashMap;

mod monitor;

/**
  Driver
*/
pub struct Driver {
    pub name: &'static str,
    // heterogenous hashmap of Trait Objects
    pub component_db: HashMap<String, &'static mut dyn PhasingA>,
    pub phase: verif::Phase,
    pub vif: verif::VirtualInterface,
    pub seq_item_port: verif::TlmPort<u32>,
    pub rsp_port: verif::TlmPort<u32>,
}

impl Objectify for Driver {
    fn get_name(&self) -> String {
        self.name.to_string()
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

/**
  Sequencer
*/
mod sequencer {
    pub struct Sequencer {
        pub name: &'static str,
    }
}

/**
  Agent
*/

pub struct Agent {
    pub name: &'static str,
    // heterogenous hashmap of Trait Objects
    pub component_db: HashMap<String, Box<dyn PhasingA>>,
    pub phase: verif::Phase,
    pub is_active: bool,
}
impl Objectify for Agent {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl PhasingA for Agent {
    fn get_phase(&self) -> verif::Phase {
        dbg!("{}", &self.phase);
        println!("{}: {}", self.get_name(), &self.phase);
        self.phase.clone()
    }

    fn configure(&mut self) {
        // allocate the child monitor and driver

        self.component_db.insert(
            String::from("kjsdkfjk"),
            Box::new(monitor::Monitor {
                name: "sldkfj",
                component_db: HashMap::new(),
                phase: self.phase.clone(),
                vif: VirtualInterface {},
            }),
        );

        if self.is_active {
            let drvr = Driver {
                name: "jklwej",
                component_db: HashMap::new(),
                phase: self.phase.clone(),
                vif: VirtualInterface {},
                rsp_port: TlmPort { it: 3 },
                seq_item_port: TlmPort { it: 4 },
            };
            self.component_db.insert(drvr.get_name(), Box::new(drvr));
        }

        // Top-Down configuration
        for v in self.component_db.values_mut() {
            v.configure();
        }
    }
}
