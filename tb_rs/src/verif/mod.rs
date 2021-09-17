//!
//! DSL library for writing verification testbenches
//!

use std::fmt;

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

/// Transaction-Level-Modeling
pub trait Tlm<T> {
    fn get_next_item(&self) -> T;
}

pub struct TlmPort<T> {
    pub it: T,
}

impl<T> Tlm<T> for TlmPort<T> {
    fn get_next_item(&self) -> T {
        todo!()
    }
}
