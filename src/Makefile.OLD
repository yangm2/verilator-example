TOP_MODULE ?= hello_world

default: obj_dir/V$(TOP_MODULE)

PLUS_ARGS ?= +MAX_CYCLES=100

obj_dir/V$(TOP_MODULE).mk: example_ifc.sv sim_main.cpp example_vpi.cpp
	/usr/local/bin/verilator -Wall -cc $(TOP_MODULE).sv --exe $^

obj_dir/V$(TOP_MODULE): obj_dir/V$(TOP_MODULE).mk
	/usr/bin/make -j -C obj_dir -f V$(TOP_MODULE).mk V$(TOP_MODULE)

.phony run: obj_dir/V$(TOP_MODULE)
	obj_dir/V$(TOP_MODULE) $(PLUS_ARGS)

.phony clean:
	/bin/rm -rf obj_dir