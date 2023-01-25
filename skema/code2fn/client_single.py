#!/usr/bin/env python

"""Example Python client program to communicate with the Code2FN service."""

import os
import json
import requests
import argparse


def system_to_json(
    file_path: str 
) -> str:
    
    with open(file_path, "r") as f:
        blob = f.read()

    system_name = os.path.basename(file_path).strip(".py")
    
    return json.dumps(
        {
            "files": [system_name],
            "blobs": [blob],
            "system_name": system_name,
            "root_name": ""  
        }
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--url",
        default="http://localhost:8000/fn-given-filepaths",
        help="Host machine where the Code2FN service is running",
    )

    parser.add_argument(
        "--write",
        action="store_true",
        help=(
            "If this flag is provided, the program writes the response "
            "to a file. Otherwise it prints the response to standard output."
        ),
    )

    parser.add_argument("file_path", type=str)

    args = parser.parse_args()

    data = system_to_json(
        args.file_path
    )
    response = requests.post(args.url, data=data)

    system_name = json.loads(data)["system_name"]
    if args.write:
        with open(f"{system_name}--Gromet-FN-auto.json", "w") as f:
            f.write(response.json())
    else:
        print(response.json())