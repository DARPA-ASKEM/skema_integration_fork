import argparse
import tempfile
import os

from skema.program_analysis.multi_file_ingester import process_file_system

def process_file(path: str):
    system_name = os.path.basename(path).strip(".py")
    root_path = os.path.dirname(path)

    # Create temporary system_filepaths file
    tmp = tempfile.NamedTemporaryFile(mode="w", delete=False)
    tmp.write(os.path.basename(path))
    tmp.close()

    process_file_system(system_name, root_path, tmp.name, True)

    # Delete temporary system_filepaths file
    os.unlink(tmp.name)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--path", type=str, help="The path of source directory"
    )
    args = parser.parse_args()
    process_file(args.path)
    