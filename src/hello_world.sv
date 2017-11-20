// Simulation code (DUT & parts of testbench)

// rudimentary top-level module with some logic+state
// * use basic||synthesizable SystemVerilog syntax
module hello_world(
    input bit clk,
    input bit reset,
    // drive outputs from testbench-driver
    output bit a_valid,
    output bit [3:0] a_opcode,
    output bit [1:0] a_beat,
    output bit [7:0] a_data
);
    timeunit 1ns; // _verilator_ ignores these
    timeprecision 1ns;

    // verilator lint_off UNUSED
    // verilator lint_off UNDRIVEN
    int unsigned line = `__LINE__;  
    // verilator lint_on UNDRIVEN
    // verilator lint_on UNUSED

    int unsigned max_cycles = 32'd10;
    initial begin
        $display("hello world");
        if($value$plusargs("MAX_CYCLES=%d", max_cycles)) begin
            $display("-I- found MAX_CYCLES=%d", max_cycles);
        end
        else begin
            $display("-I- using default MAX_CYCLES=%d", max_cycles);
        end
    end

    int unsigned counter = 32'b0;
    always_ff @(posedge clk) begin
        if (reset) begin
            counter <= 32'd0;
        end
        else begin
            counter <= counter + 1'b1;
        end
    end

    // simple test-end condition
    always_comb begin
        if (counter > max_cycles) begin
            $finish;
        end
    end

    // verilator coverage_off
    import "DPI-C" function integer add (input integer a, input integer b);
    initial begin
        $display("%x + %x = %x", line, 2, add(line,2));
    end
    // verilator coverage_on

endmodule : hello_world