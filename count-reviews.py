#!/usr/bin/env python3

import argparse


def count_reviews(repo, users):
  return {"repository": repo, "users": users}


def parse_args():
  parser = argparse.ArgumentParser(description="Count reviews on a GitHub repository")
  parser.add_argument("repo", help="GitHub repository in format owner/repo")
  parser.add_argument(
      "--users",
      help="Comma-separated usernames to filter reviews for",
      type=lambda s: tuple(u.strip() for u in s.split(",")),
  )

  return parser.parse_args()

def main():
  args = parse_args()
  print(count_reviews(args.repo, args.users))

if __name__ == "__main__":
  main()
