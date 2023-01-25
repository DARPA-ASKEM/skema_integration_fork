import argparse
import tempfile
import os

from skema.utils.script_functions import process_file

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--path", type=str, help="The path of source file"
    )
    args = parser.parse_args()
    process_file(args.path, True)