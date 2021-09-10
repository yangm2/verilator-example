// template for a SV-collector which calls a C-monitor

interface example_passive_ifc (
    // verilator lint_off UNUSED
    input bit clk,
    // verilator lint_on  UNUSED
    input int unsigned cycle_count
);
    // int unsigned cycle_count;

    // modport mon (input cycle_count);
    // modport drv (output cycle_count);

    // NOTE: https://www.veripool.org/issues/1118-Verilator-Clocking-block
    // clocking cb @(posedge clk);
    //     input cycle_count;
    // endclocking
    // default clocking cb;

    import "DPI-C" function void c_monitor (input int a);
    // NOTE: optimized so that it triggers only when cycle_count changes
    always @(cycle_count) begin
        c_monitor(cycle_count);
    end

    // 
    final begin
        $display("I am %m");
    end

endinterface : example_passive_ifc

// instantiate SV-collector in DUT with bind
// verilator lint_off UNUSED
bind hello_world example_passive_ifc bind_passive_ifc (.clk(clk), .cycle_count(counter));
// verilator lint_on UNUSED
