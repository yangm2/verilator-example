// template for a SV-driver which pulls from a C-sequencer

interface example_active_ifc (
    // verilator lint_off UNUSED
    input bit clk,
    output bit valid_00H,
    output bit [3:0] opcode_01H, // TODO: global enum?
    output bit [1:0] beat_0nH, // TODO: global enum?
    output bit [7:0] data_0nH
    // verilator lint_on  UNUSED
);

    // TODO: deserialize transactions from C
    // blocking transaction-getter
    // drive pipelined/folded interface
    // cycle  |   00   |   01   |   02   |   03   |   04   |   05   |   06   |   07   |   08   |   09  |
    // valid  |   T0   |                              T1
    // opcode |        |   T0   |                              T1
    // beat   |        |        | T0=00  | T0=01  | T0=10  | T0=11  | T1=00  | T1=01  | T1=10  | T1=11 |
    // data   |        |        |  T0.0  |  T0.1  |  T0.2  |  T0.3  |  T1.0  |  T1.1  |  T1.2  |  T1.3 |

    // NOTE: without clocking blocks, must manually unpack into a pipeline

    // FIXME: temporary assignments
    assign valid_00H = 1'b0;
    assign opcode_01H = 4'ha;
    assign beat_0nH = 2'b00;
    assign data_0nH = 8'h0e;

    // demonstrate final-block & display %m
    final begin
        $display("FINALLY: I am %m");
    end

endinterface : example_active_ifc

// instantiate SV-collector in DUT with bind
// verilator lint_off UNUSED
bind hello_world example_active_ifc i_bind_active_ifc (.clk(clk), 
                                                       .valid_00H(a_valid), 
                                                       .opcode_01H(a_opcode),
                                                       .beat_0nH(a_beat),
                                                       .data_0nH(a_data));
// verilator lint_on UNUSED
