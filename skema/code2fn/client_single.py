#!/usr/bin/env python

"""Example Python client program to communicate with the Code2FN service."""

import os
import json
import requests
import argparse


def module_to_json(
    file_path: str 
) -> str:
    
    with open(file_path, "r") as f:
        blob = f.read()

    return json.dumps(
        {
            "file": file_path,
            "blob": blob
        }
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--host",
        default="localhost",
        help="Host machine where the Code2FN service is running",
    )
    parser.add_argument(
        "--port",
        type=int,
        default=8000,
        help="Port on which the Code2FN service is running",
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

    url = f"http://{args.host}:{args.port}/single"
    data = module_to_json(args.file_path)
    response = requests.post(url, data=data)

    if args.write:
        with open(f"{args.system_name}--Gromet-FN-auto.json", "w") as f:
            f.write(response.json())
    else:
        print(response.json())
