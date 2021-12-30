#!/usr/bin/env python3

"""
YuzuRyo61 - Name regex
"""

import re
import argparse

argp = argparse.ArgumentParser(
    description="Check name which is YuzuRyo61"
)
argp.add_argument("name", type=str, help="A name for checking")

# See name-rule.md
yuzuryo61_regex = re.compile("(YuzuRyo|yuzuryo|YUZURYO)(61)?|ゆずりょー?")


def check_regex(check_string: str) -> bool:
    return yuzuryo61_regex.fullmatch(check_string) is not None


if __name__ == "__main__":
    args = argp.parse_args()
    result = check_regex(args.name)
    if result:
        print(f"{args.name} is YuzuRyo61 namespace")
    else:
        print(f"{args.name} is not YuzuRyo61 namespace")
