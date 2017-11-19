// Wrapper for (System)Verilog, SystemC/C/C++ & simulator
// * simulator boiler-plate (i.e. drive reset/clk)
// * process/passthru cmdline args (i.e. wrapper args, plus args)
// TODO:
// * randomization/determinism (i.e. seed initialization)
// * SystemVerilog coverage?
// * SystemC/C/C++ code-coverage?

#include "Vhello_world.h"
#include "verilated.h"

// This is a 64-bit integer to reduce wrap over issues and
// allow modulus.  You can also use a double, if you wish.
vluint64_t main_time = 0;

// Called by $time in Verilog
// converts to double, to match
// what SystemC does
double sc_time_stamp () {
    return main_time;
}

int main(int argc, char** argv, char** env) {
    Verilated::commandArgs(argc, argv);
    Vhello_world* top = new Vhello_world;
    while (!Verilated::gotFinish()) {
        top->reset = (main_time < 5) ? 1 : 0;
        top->clk = main_time % 2;
        top->eval();
        main_time++;
    }
    top->final();
    delete top;
    exit(0);
}