use std::{collections::HashMap, error::Error, ffi::CStr, os::raw::c_char};

// TODO: move this up to WORKSPACE
/// includes verif/mod.rs
pub mod verif;

/**
  example of a standalone interface Agent
*/
mod my_agent;

/**
 example of common environment for tests
*/
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

/**
 Example of a test
*/
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

/**
 Testbench-function tests (Testing the testbench, not the DUT)
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
