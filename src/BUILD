genrule(
    name = "verilator",
    srcs = ["hello_world.sv", "example_ifc.sv", "sim_main.cpp", "example_vpi.cpp"],
    outs = ["Vhello_world.mk"],
    local = 1,
    cmd = "/usr/local/bin/verilator -Wall -Mdir $(@D) -cc --exe $(SRCS)",
    tools = [],
)

# FIXME: could separately compile example_vpi.o and just do link in Vhello_world.mk
genrule(
    name = "make-Vhello_world",
    srcs = [":verilator"],
    outs = ["Vhello_world"],
    local = 1,
    executable = 1,
    cmd = "/usr/bin/make -j -C $(@D) --makefile Vhello_world.mk --print-directory VM_USER_DIR=../../../..",
    tools = [],
)
