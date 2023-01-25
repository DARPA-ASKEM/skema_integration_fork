import argparse

from skema.utils.script_functions import process_file_system

def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--sysname", type=str, help="The name of the system we're ingesting"
    )
    parser.add_argument(
        "--path", type=str, help="The path of source directory"
    )
    parser.add_argument(
        "--files",
        type=str,
        help="The path to a file containing a list of files to ingest",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="If true, the script write the output to a JSON file"
    )

    options = parser.parse_args()
    return options


if __name__ == "__main__":
    args = get_args()

    system_name = args.sysname
    path = args.path
    files = args.files

    print(f"Ingesting system: {system_name}")
    print(f"With root directory as specified in: {path}")
    print(f"Ingesting the files as specified in: {files}")

    process_file_system(system_name, path, files, args.write)
