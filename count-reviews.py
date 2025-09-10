#!/usr/bin/env python3

import argparse


def main():
    parser = argparse.ArgumentParser(description="Count reviews on a GitHub repository")
    parser.add_argument("repo", help="GitHub repository in format owner/repo")

    args = parser.parse_args()

    print(f"Repository: {args.repo}")


if __name__ == "__main__":
    main()
