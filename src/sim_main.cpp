// Wrapper for (System)Verilog, SystemC/C/C++ & simulator
// * simulator boiler-plate (i.e. drive reset/clk)
// * process/passthru cmdline args (i.e. wrapper args, plus args)
// * tracing to FST
// TODO:
// * randomization/determinism (i.e. seed initialization)
// * SystemVerilog coverage?
// * SystemC/C/C++ code-coverage?

#include "Vhello_world.h"
#include "verilated.h"
#include "verilated_fst_c.h"

// This is a 64-bit integer to reduce wrap over issues and
// allow modulus.  You can also use a double, if you wish.
vluint64_t main_time = 0;

// Called by $time in Verilog
// converts to double, to match
// what SystemC does
double sc_time_stamp () {
    return main_time;
}

int main(int argc, char** argv) {
    const std::unique_ptr<VerilatedContext> contextp{new VerilatedContext};
    const std::unique_ptr<Vhello_world> top{new Vhello_world{contextp.get(), "i_root"}};

    VerilatedFstC* tfp = new VerilatedFstC;

    contextp->commandArgs(argc, argv);
    contextp->traceEverOn(true);

    top->trace(tfp, 99); // Trace 99 levels of hierarchy
    tfp->open("runsim.fst");

    while (!contextp->gotFinish()) {
        contextp->timeInc(1);

        // Toggle a fast (time/2 period) clock
        top->clk = !top->clk;

        // Evaluate model
        top->eval();
        tfp->dump(contextp->time());
    }
    top->final();

    tfp->close();

    // Final simulation summary
    contextp->statsPrintSummary();

    exit(0);
}