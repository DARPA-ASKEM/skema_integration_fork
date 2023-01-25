#!/usr/bin/env python

"""Example Python client program to communicate with the Code2FN service."""

import os
import json
import requests
import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--url",
        default="http://localhost:8000/fn-given-filepaths-zip",
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

    file = {"file": open(args.file_path, "rb")}
    response = requests.post(args.url, files=file)

    if args.write:
        with open(f"{args.system_name}--Gromet-FN-auto.json", "w") as f:
            f.write(response.json())
    else:
        print(response.json())