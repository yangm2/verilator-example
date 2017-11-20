# Intro
This project ties together a few interesting technologies:
* verilator
* SystemVerilog (for DUT & TB)
* C/C++ (for TB & DUT)
* Bazel (for building)
* **TODO**: rust (for TB & DUT, maybe sim_main/wrapper/cmd-line-processing)

# Components
## DUT
* counter
* valid-interface (**TODO**: add flow-control/back-pressure)

## Testbench
* collector + monitor - demonstrate SV+VPI(C/C++)
* driver + sequencer - demonstrate SV+VPI(C/C++) for pipelined interface (**TODO**: add flow-control/back-pressure)

# Commands
* Clean:

  `bazel clean`
* Compile:

  `bazel build //src:Vhello_world`
* Run:

  `bazel run //src:Vhello_world +MAX_CYCLES=23`