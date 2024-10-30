#! /usr/bin/env -S uv run

# quick and dirty Python/UV script to get the prelude_hash corresponding to the
# nix-managed `buck2` release
#
# pre-req: this script assumes `buck2` was installed with
#          [`nix`](https://github.com/NixOS/nixpkgs/tree/nixpkgs-unstable/pkgs/development/tools/build-managers/buck2)
# pre-req: this script requires [uv](https://github.com/astral-sh/uv) in order
#          to run (and install dependencies)

# *********************************************************
# ** inline UV dependency install as metadata script ... **
# *********************************************************
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "requests",
# ]
# ///

import shutil
import pathlib
import re
import requests

if pstr := shutil.which("buck2"):
    p = pathlib.Path(pstr)
    pabs = p.resolve(strict=True)
    nix_store_dir = pabs.parts
    calver_re = re.compile(r"\w+-buck2-(un)?stable-(\d\d\d\d-\d\d-\d\d)")
    for part in nix_store_dir:
        (stable, calver) = m.groups() if (m := calver_re.match(part)) else (None, None)
        if calver is not None:
            print(
                f"nix installed version of {stable}stable 'buck2' is [{calver}] at [{pstr}]"
            )
            r = requests.get(
                f"https://github.com/facebook/buck2/releases/download/{calver}/prelude_hash"
            )
            prelude_hash_val = r.text.rstrip()
            print(f"corresponding 'prelude_hash' is [{prelude_hash_val}]")
