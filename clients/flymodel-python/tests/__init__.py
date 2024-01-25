from pathlib import Path
from sys import path

src = Path(__file__).parent.parent / "src" / "flymodel_client"

assert src.exists() and src.is_dir()

path.append(str(src))
