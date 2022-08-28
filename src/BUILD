genrule(
    name = "verilator",
    srcs = [
        "hello_world.sv",
        "example_passive_ifc.sv",
        "example_active_ifc.sv",
        "sim_main.cpp",
        "example_vpi.cpp",
    ],
    outs = [
        "Vhello_world.mk",
        "Vhello_world.h",
        "Vhello_world.cpp",
        "Vhello_world__Syms.h",
        "Vhello_world___024root.h",
        "Vhello_world___024unit.h",
        "Vhello_world_example_passive_ifc.h",
        "Vhello_world_example_active_ifc.h",
        "Vhello_world__Dpi.h",
        "Vhello_world__Dpi.cpp",
    ],
    cmd = """
        verilator \
            -Wall \
            -Mdir $(RULEDIR) \
            -cc \
            --exe \
            $(SRCS)
    """,
    exec_tools = [],
    local = 1,
)

# FIXME: could separately compile example_vpi.o and just do link in Vhello_world.mk
genrule(
    name = "make-Vhello_world",
    srcs = [
        ":verilator",
        "sim_main.cpp",
        "example_vpi.cpp",
    ],
    outs = ["Vhello_world"],
    cmd = """
        MAKEFILE=`echo $(locations //src:verilator) | grep -o --regexp "\\w\\+.mk" `
        CMDDIR=`pwd`
        /usr/bin/make \
            -j \
            -C $(RULEDIR) \
            --makefile $$MAKEFILE \
            --print-directory \
            VM_USER_DIR=$$CMDDIR
    """,
    exec_tools = [],
    executable = 1,
    local = 1,
)
