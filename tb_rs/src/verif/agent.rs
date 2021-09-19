//! Agent

use crate::verif::{self, Objectify, PhasingA};
use std::collections::HashMap;

pub struct Agent<Mode> {
    name: String,
    // heterogenous hashmap of Trait Objects
    pub component_db: HashMap<String, Box<dyn PhasingA>>,
    phase: verif::Phase,
    mode: Mode,
}

impl<M> Objectify for Agent<M> {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl<M> PhasingA for Agent<M> {
    fn get_phase(&self) -> verif::Phase {
        dbg!("{}", &self.phase);
        println!("{}: {}", self.get_name(), &self.phase);
        self.phase.clone()
    }

    // BTW, should this be immutable?
    // TODO: how should a configure()-implementation be injected here???  closure??!?
    fn configure(&mut self) {
        todo!()
    }
}

pub fn new(name: &str) -> Agent<super::PassiveMode> {
    Agent {
        name: name.to_string(),
        component_db: HashMap::new(),
        phase: super::Phase::Allocated,
        mode: super::PassiveMode,
    }
}

impl Agent<super::PassiveMode> {
    /// convert PassiveMode Agent to ActiveMode Agent
    pub fn to_active(self) -> Agent<super::ActiveMode> {
        Agent {
            name: self.name.into(),
            component_db: self.component_db.into(),
            phase: self.phase.into(),
            mode: super::ActiveMode,
        }
    }
}

// FIXME: this feels clunky ... Trait???
impl Agent<super::ActiveMode> {
    pub fn is_active(&self) -> bool {
        true
    }
}

impl Agent<super::PassiveMode> {
    pub fn is_active(&self) -> bool {
        false
    }
}

impl super::configuration::Configurable<Agent<super::ActiveMode>> for Agent<super::ActiveMode> {
    fn config(
        self,
        c: verif::configuration::Configuration<Agent<super::ActiveMode>>,
    ) -> Agent<super::ActiveMode> {
        todo!()
    }
}
