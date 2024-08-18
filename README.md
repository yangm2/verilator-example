# Intro
This project ties together a few interesting technologies:
* [verilator](https://www.veripool.org/verilator/)
* SystemVerilog (for DUT & TB)
* C/C++ (for TB & DUT)
* [Buck2](https://buck2.build/) (for building)
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
  * `buck2 clean`

* Compile:
  * `buck2 build //src:Vhello_world`

* Run:
  * `buck2 run //src:Vhello_world +MAX_CYCLES=23`
  * `buck2 run //src:runsim`
  