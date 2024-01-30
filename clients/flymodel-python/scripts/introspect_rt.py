from pathlib import Path

from flymodel_client import models

path = Path(__file__).parent.parent / "src" / "flymodel" / "models"


def model_import_stmt_pyi(name: str):
    return f"import flymodel.models.{name} as {name}"


def imports_for_mod(mod):
    return list(filter(lambda v: not v.startswith("__"), dir(mod)))


def introspect_models():
    top_level = imports_for_mod(models)

    pyi = ""
    for mod in top_level:
        pyi += model_import_stmt_pyi(mod) + "\n"

    with open(path / "__init__.pyi", "w") as f:
        f.write(pyi)

    for mod in top_level:
        pyi = path / (mod + ".pyi")
        if (pyi).exists():
            print(f"skipping {mod}")
            continue
        module = getattr(models, mod)
        imports = imports_for_mod(module)
        doc = getattr(module, "__doc__", "")
        spec = getattr(module, "__spec__", "")

        stub = f"""
__all__ = {imports!r}
__doc__ = {doc!r}
__spec__ = {spec!r}
"""
        with open(pyi, "w") as f:
            f.write(stub)

    print()


if __name__ == "__main__":
    assert path.exists()
    assert path.is_dir()
    introspect_models()
