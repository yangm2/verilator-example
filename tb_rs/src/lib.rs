use std::{collections::HashMap, error::Error, ffi::CStr, os::raw::c_char};

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

mod verif {
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
}

mod my_agent {
    use crate::verif::{self, Objectify, PhasingA, PhasingB, VirtualInterface};
    use std::collections::HashMap;
    pub struct Agent {
        pub name: &'static str,
        // heterogenous hashmap of Trait Objects
        pub component_db: HashMap<String, Box<dyn PhasingA>>,
        pub phase: verif::Phase,
        pub is_active: bool,
    }

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

    pub struct Driver {
        pub name: &'static str,
        // heterogenous hashmap of Trait Objects
        pub component_db: HashMap<String, &'static mut dyn PhasingA>,
        pub phase: verif::Phase,
        pub vif: verif::VirtualInterface,
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
                Box::new(Monitor {
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
                };
                self.component_db.insert(drvr.get_name(), Box::new(drvr));
            }

            // Top-Down configuration
            for v in self.component_db.values_mut() {
                v.configure();
            }
        }
    }
}

mod my_env {

    use crate::verif::{self, Objectify, PhasingA, PhasingB};
    use std::collections::HashMap;

    pub struct Env {
        pub name: &'static str,
        // heterogenous hashmap of Trait Objects
        pub component_db: HashMap<String, Box<dyn PhasingA>>,
        phase: verif::Phase,
    }

    impl Objectify for Env {
        fn get_name(&self) -> String {
            self.name.to_string()
        }
    }

    impl PhasingA for Env {
        fn get_phase(&self) -> verif::Phase {
            dbg!("{}", &self.phase);
            println!("{}: {}", self.get_name(), &self.phase);
            self.phase.clone()
        }

        fn configure(&mut self) {
            use crate::my_agent;

            // Add an agent to the environment
            let a = my_agent::Agent {
                name: "lkjsdl",
                component_db: HashMap::new(),
                phase: self.phase.clone(),
                is_active: true,
            };

            self.component_db
                .insert(String::from("agent1"), Box::new(a));

            // Top-Down configuration
            for v in self.component_db.values_mut() {
                v.configure();
            }
        }
    }

    impl PhasingB for Env {
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
}

use crate::verif::PhasingA;

struct Test1 {
    name: &'static str,
    component_db: HashMap<String, &'static dyn PhasingA>,
}

// impl Default for Test1 {
//     /// define a testbench, like a tree-datastructure, top-down (root to leaves)
//     fn default() -> Self {
//         let mut d = Test1 {
//             name: "default_name",
//             component_db: HashMap::new(),
//         };

//         // declaration and allocation
//         let e3 = My_Env::Env {
//             name: "foo",
//             component_db: HashMap::new(),
//         };

//         //
//         d.component_db
//             .insert("asdf".to_string(), Verif::ComponentBuilder(e3));

//         let e1 = Component::from(Env { name: "sjdkf" })
//             .into_connect()
//             .into_endofelaboration()
//             .into_component();

//         let e2 = Component::from(Env { name: "ksdjlf" })
//             .into_connect()
//             .into_endofelaboration()
//             .into_component();

//         // This is silly ... returns the same thing that was passed in
//         let t = Component::from(Test1 {
//             name: "asdf",
//             component_db: HashMap::new(),
//         })
//         .into_connect()
//         .into_endofelaboration()
//         .into_component();

//         t
//     }
// }

impl Test1 {
    fn run(b: bool) -> Result<bool, Box<dyn Error>> {
        Ok(b)
    }
}

struct Test2 {
    name: &'static str,
}

impl Test2 {
    fn run() -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }
}

enum Tests {
    Test1,
    Test2,
}

#[no_mangle]
/// like uvm_pkg::run_test(), takes test name
// TODO: figure out return type to C
pub extern "C" fn run_test(test_name: *const c_char) -> () {
    if test_name.is_null() {
        // return Result::Err;
    }
    let input = unsafe { CStr::from_ptr(test_name).to_str() };

    // TODO: replace with Tests-enum pattern match?
    match input {
        Err(_) => todo!(),
        Ok("Test1") => Test1::run(false),
        Ok("Test2") => Test2::run(),
        _ => Err(Box::from(input.unwrap())),
    };

    ()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
