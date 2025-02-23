#!/usr/bin/env python

"""
Reads a Tab Session Manager export JSON file and
converts it to a format that is more useable for
reading by a parser in a statically typed language.
"""

import argparse
import json


def convert_window(win):
    """
    The window is an object with sequential integer
    keys, and this converts it to a list of objects.
    """

    window = {"tabs": []}
    tabs = window["tabs"]
    for _, tab in win.items():
        tabs.append(tab)

    return window


def convert_windows(windows) -> list:
    """
    Windows is an object with sequential integer keys,
    and this converts it to a list of objects.
    """

    win_list = []
    for _, win in windows.items():
        win_list.append(convert_window(win))

    return win_list


def run(file_name: str):
    with open(file_name, "r") as file:
        j_file: list = json.load(file)

    for session in j_file:
        windows = session["windows"]
        session["windows"] = convert_windows(windows)

    return j_file


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Restructure Tab Session Manager JSON to use lists instead of objects with integer keys."
    )
    parser.add_argument(
        "--in-file",
        dest="in_file",
        type=str,
        required=True,
        help="Name of file to convert.",
    )
    parser.add_argument(
        "--out-file",
        dest="out_file",
        type=str,
        required=True,
        help="Name of file to write result to.",
    )
    args = parser.parse_args()

    processed_object = run(args.in_file)
    with open(args.out_file, "w") as file:
        json.dump(processed_object, file, indent=2)
