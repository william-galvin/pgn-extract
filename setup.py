from setuptools import setup
from setuptools.command.build import build as _build
import subprocess
import pathlib

class BuildWithMake(_build):
    def run(self):
        project_root = pathlib.Path(__file__).parent.resolve()
        subprocess.check_call(["make"], cwd=project_root)
        super().run()

setup(
    name="pgn-extract",
    version="0.0.1",
    cmdclass={"build": BuildWithMake},
    data_files=[("bin", ["pgn-extract"])],
)
