//!
//! DSL library for writing verification testbenches
//!

use std::fmt;

pub mod sequencer;
pub mod tlm;

#[derive(Clone)]
pub struct VirtualInterface;

pub trait Objectify {
    fn get_name(&self) -> String;
}

pub trait PhasingA {
    fn get_phase(&self) -> Phase;

    /// Top-Down configuration
    fn configure(&mut self);
}

pub trait PhasingB {
    fn connect<T: PhasingB>(&self, other: T);
    fn elaborate(&self);
    /**
     TODO: need concept of "time" in order to distinguish between
     non-time-consuming and time-consuming fn's
    */
    /*async*/
    fn run(&self);
    fn shutdown(&self);
}

#[derive(Clone, Debug)]
pub enum Phase {
    Allocated,
    Configured,
    Connected,
    Run,
}

// println! for Phase
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Phase::Allocated => write!(f, "Allocated"),
            Phase::Configured => write!(f, "Configured"),
            Phase::Connected => write!(f, "Connected"),
            Phase::Run => write!(f, "Run"),
        }
    }
}

// pub enum Components {
//     Environment(Object),
//     Agent,
//     Monitor,
//     Driver,
//     Sequencer,
// }

// // this is like a C function-pointer
// pub enum Object {
//     Component(u32),
//     Transaction,
// }

// impl Object {
//     fn unref(&self) -> Components {
//         match self {
//             Object::Component(i) => Components::Environment(*i),
//             _ => Components::Agent,
//         }
//     }
// }

// pub struct Component<PHASE, OBJ> {
//     phase: PHASE,
//     object: OBJ,
// }

// pub fn ComponentBuilder<OBJ>(obj: OBJ) -> Component<Phase, Components> {
//     let a = Object::Component(1);

//     Component {
//         phase: Phase::Allocated,
//         object: a.unref(),
//     }
// }

// mod Component {
//     use std::error::Error;

//     struct Constructor<PHASE, OBJ> {
//         phase: PHASE,
//         object: OBJ,
//     }

//     struct BuildPhase;
//     struct ConnectPhase;
//     struct EndOfElaborationPhase;

//     struct Frozen;

//     pub fn from<OBJ>(obj: OBJ) -> Constructor<BuildPhase, OBJ> {
//         Constructor {
//             phase: BuildPhase,
//             object: obj,
//         }
//     }

//     impl<P, O> Constructor<P, O> {}

//     impl<OBJ> Constructor<BuildPhase, OBJ> {
//         pub fn into_connect(self) -> Constructor<ConnectPhase, OBJ> {
//             Constructor {
//                 phase: ConnectPhase,
//                 object: self.object,
//             }
//         }
//     }

//     impl<OBJ> Constructor<ConnectPhase, OBJ> {
//         pub fn into_endofelaboration(self) -> Constructor<EndOfElaborationPhase, OBJ> {
//             Constructor {
//                 phase: EndOfElaborationPhase,
//                 object: self.object,
//             }
//         }
//     }

//     impl<OBJ> Constructor<EndOfElaborationPhase, OBJ> {
//         pub fn into_component(self) -> OBJ {
//             self.object
//         }
//     }
// }

// struct Agent {
//     name: &'static str,
//     mon: Monitor,
//     drvr: Driver,
//     seqr: Sequencer,
// }

// struct Env {
//     name: &'static str,
// }
