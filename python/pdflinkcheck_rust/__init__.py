# pdflinkcheck_rust/__init__.py
import os
import sys

_package_dir = os.path.dirname(os.path.abspath(__file__))

if sys.platform == "win32":
    # Mandatory for Windows
    if hasattr(os, "add_dll_directory"):
        os.add_dll_directory(_package_dir)
    else:
        os.environ["PATH"] = _package_dir + os.pathsep + os.environ.get("PATH", "")

# We import the rust binary here. 
# Because of RPATH (Linux) and add_dll_directory (Win), it will find libpdfium.
from .pdflinkcheck_rust import analyze_pdf

__all__ = ["analyze_pdf"]
