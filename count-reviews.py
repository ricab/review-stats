#!/usr/bin/env python3

import argparse


def main():
    parser = argparse.ArgumentParser(description="Count reviews on a GitHub repository")
    parser.add_argument("repo", help="GitHub repository in format owner/repo")
    parser.add_argument("--users", help="Comma-separated usernames to filter reviews for")

    args = parser.parse_args()

    print(f"Repository: {args.repo}")
    if args.users:
        users_list = [user.strip() for user in args.users.split(",")]
        print(f"User filter: {users_list}")


if __name__ == "__main__":
    main()
