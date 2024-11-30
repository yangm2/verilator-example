# BUCK

genrule(
    name = "hello_world",
    out = "out.txt",
    cmd = "echo BUILT BY BUCK2> $OUT",
)

# alias that reaches into //src
# (exe target must be "visible" to this alias)
command_alias(
    name = "sim101",
    exe = "//src:Vhello_world",
    args = [
        "+MAX_CYCLES=101",
    ],
)
