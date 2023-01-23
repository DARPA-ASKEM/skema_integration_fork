import argparse
import glob
import os

from skema.utils.fold import dictionary_to_gromet_json, del_nulls

from single_file_ingester import process_file

# Will run single file ingestor on every .py file in a directory
parser = argparse.ArgumentParser()
parser.add_argument(
    "--root_path", type=str, help=""
)
parser.add_argument(
    "--version", type=str, help=""
)
args = parser.parse_args()

source_files = glob.glob(f"{args.root_path}/**/*.py", recursive=True) # Find all .py files in directory and recurse through subdirectories
for file_path in source_files:
    gromet_collection = process_file(file_path, False)
    
    # Create gromet folder alongside source code
    source_dir = os.path.dirname(file_path).replace(".py", "")
    gromet_dir = os.path.join(source_dir, args.version)
    if not os.path.exists(gromet_dir):
        os.mkdir(gromet_dir)

    # Write gromet to file
    filename = f"{os.path.basename(os.path.normpath(file_path))}--Gromet-FN-auto.json".replace(".py", "")
    output_file_path = os.path.join(gromet_dir, filename)
    with open(output_file_path, "w") as f:
        f.write(dictionary_to_gromet_json(del_nulls(gromet_collection.to_dict())))