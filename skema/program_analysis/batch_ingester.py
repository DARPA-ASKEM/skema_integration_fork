import argparse
import glob

from single_file_ingester import process_file

# Will run single file ingestor on every .py file in a directory
parser = argparse.ArgumentParser()
parser.add_argument(
    "--root_path", type=str, help=""
)
args = parser.parse_args()

source_files = glob.glob(f"{args.root_path}/**/*.py", recursive=True) # Find all .py files in directory and recurse through subdirectories
for file_path in source_files:
    process_file(file_path)