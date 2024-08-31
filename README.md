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
  * `buck2 run //src:Vhello_world +MAX_CYCLES=23`
  * `buck2 run //src:runsim`

## Maintenance

### Buck2

* `prelude` submodule version needs to match (or be behind?) the corresponding installed `buck2` version
  1. `buck2 --version` -- doesn't actually tell you anything useful
  1. `which buck2` -- path name may give a clue to installed (prerelease) version
     * on MacOS `nix` managed ```readlink `which buck2` ``` e.g. yields `/nix/store/0x6w5fqzvlbwla7dsq8n2bjimkqsqdm7-buck2-unstable-2024-05-15/bin/buck2`
  1. refer to [buck2 releases on GitHub](https://github.com/facebook/buck2/releases)
     * in the above example, look at [2024-05-15](https://github.com/facebook/buck2/releases/tag/2024-05-15)
  1. download `prelude_hash` from assets
  1. in repo ...
    ```sh
    cd prelude
    git checkout `cat $PATH_TO/prelude_hash`
    cd `git rev-parse --show-toplevel`
    git add prelude
    git commit -m 'bump buck2-prelude submodule for buck2 2024-05-15'
    git push
    ```