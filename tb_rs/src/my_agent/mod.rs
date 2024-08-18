//!
//! First-party definition of a testbench agent
//!

use crate::verif::{self, tlm::Tlm, ActiveMode, Objectify, PhasingA};

/**
  Agent with stuff in it (e.g. Monitor, Driver, Sequencer)
*/

/// re-export verif::agent::* APIs
pub use crate::verif::agent::{new, Agent};


/// trait to Overload (with specialization?) specific APIs
/// NOTE: "Overload" is *NOT* a Rust-keyword
pub(crate) trait Overload {
    fn configure(&mut self) {
        todo!()
    }
}

// FIXME: overloading with a trait works for a single-level, but how to inject several levels deeper (e.g. sequences, sequence-items)
// FIXME: I'm not sure this ActiveMode-generic API makes sense
impl Overload for verif::agent::Agent<ActiveMode> {
    fn configure(&mut self) {
        // allocate the child monitor and driver
        self.component_db.insert(
            String::from("kjsdkfjk"),
            Box::new(verif::monitor::new("a mon")),
        );

        if self.is_active() {
            // UVM "build_phase"
            let seqr = verif::sequencer::new("a seqr");
            let mut drvr = verif::driver::new("a drvr");
            drvr.configure(); // top-down configuration

            // UVM "connect_phase" - assign/alias export (Fn) from sequencer as drvr.seq_item_port.call()
            drvr.seq_item_port.set_callback(seqr.seq_item_export);

            // FIXME: normally not called in Agent (temporarily here to check
            // compile/types), move this code into the drvr run_phase
            let foo = drvr.seq_item_port.call();

            // FIXME: do we even need the HashMap since we are setting the key to be the "name" of the item?
            self.component_db.insert(drvr.get_name(), Box::new(drvr));
        }
        // Top-Down configuration
        for v in self.component_db.values_mut() {
            v.configure();
        }
    }
}
