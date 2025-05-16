# Verilator Example

## Intro
This project ties together a few interesting technologies:
* [verilator](https://www.veripool.org/verilator/)
* SystemVerilog (for DUT & TB)
* C/C++ (for TB & DUT)
* [Buck2](https://buck2.build/) (for building)
* **TODO**: rust (for TB & DUT, maybe sim_main/wrapper/cmd-line-processing)

## Repo

### Cloning w/ Submodules
```sh
git clone --recurse-submodules https://github.com/yangm2/verilator-example.git
```

### Pull repo
```sh
git pull --recurse-submodules
```

## Components
### DUT
* counter
* valid-interface (**TODO**: add flow-control/back-pressure)

### Testbench
* collector + monitor - demonstrate SV+VPI(C/C++)
* driver + sequencer - demonstrate SV+VPI(C/C++) for pipelined interface (**TODO**: add flow-control/back-pressure)

## Commands
* Clean:
  * `buck2 clean`

* Compile:
  * `buck2 build //src:Vhello_world`

* Run:
  * `buck2 run :sim101`  # root-level alias with hard-coded args that reaches into `./src`
  * `buck2 run //src:sim100`  # alias with hard-coded args
  * `buck2 run //src:Vhello_world +MAX_CYCLES=23`


## Maintenance

### Buck2

* `prelude` submodule version needs to match (or be behind?) the corresponding installed `buck2` version
  1. `buck2 --version` -- doesn't actually tell you anything useful for [`nix`-managed installs](https://github.com/NixOS/nixpkgs/tree/nixpkgs-unstable/pkgs/development/tools/build-managers/buck2)
  1. [./utils/get-nix-buck2-prelude_hash.py](./utils/get-nix-buck2-prelude_hash.py) ...
  ```sh
  Reading inline script metadata from `./update-buck2-prelude.py`
  nix installed version of unstable 'buck2' is [2024-10-15] at [/Users/michaelyang/.nix-profile/bin/buck2]
  corresponding 'prelude_hash' is [615f852ad43a901d8a09b2cbbb3aefff61626c52]
  ```
  3. in repo ...
    ```sh
    cd prelude
    git pull
    git checkout $__prelude_hash_FROM_ABOVE__
    cd `git rev-parse --show-toplevel`
    git add prelude
    git commit -m 'bump buck2-prelude submodule for buck2 $__UNSTABLE_VERSION_FROM_ABOVE__'
    git push
    ```