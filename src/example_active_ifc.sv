// template for a SV-driver which pulls from a C-sequencer

interface example_active_ifc (
    // verilator lint_off UNUSED
    input bit clk,
    // verilator lint_on  UNUSED
    input int unsigned cycle_count
);

    import "DPI-C" function void c_monitor (input integer a);
    // NOTE: optimized so that it triggers only when cycle_count changes
    always @(cycle_count) begin
        c_monitor(cycle_count);
    end

    // 
    final begin
        $display("I am %m");
    end

endinterface : example_active_ifc

// instantiate SV-collector in DUT with bind
// verilator lint_off UNUSED
bind hello_world example_active_ifc bind_active_ifc (.clk(clk), .cycle_count(counter));
// verilator lint_on UNUSED