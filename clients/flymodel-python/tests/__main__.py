import glob
import sys
from asyncio import gather, run
from importlib import import_module
from pathlib import Path

import yappi

from .fixture import _client
from .profile import Profile


async def main():
    par = Path(__file__).parent
    sys.path.append(str(par))
    opts = par.glob("*.py")
    benches = []
    for f in opts:
        mod = import_module("tests." + f.stem)
        for att in dir(mod):
            if att.startswith("vprof"):
                bench: Profile = getattr(mod, att)
                print(bench)
                assert isinstance(bench, Profile)
                benches.append(bench)

    cli = _client()

    with yappi.run():
        futs = []
        for bench in benches:
            for _ in range(bench.n_its):
                futs.append(bench.bench(cli))
            _ = await gather(*futs)
            futs.clear()

    yappi.get_func_stats().print_all()


if __name__ == "__main__":
    run(main())
