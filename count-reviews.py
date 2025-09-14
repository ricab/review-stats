#!/usr/bin/env python3

import argparse

from src.period import Period
from src.review_inspector import count_reviews
from src.parsing import parse_period


def parse_args():
  parser = argparse.ArgumentParser(description="Count reviews on a GitHub repository")
  parser.add_argument("repo", help="GitHub repository in format owner/repo")
  parser.add_argument(
    "--users",
    help="Comma-separated usernames to filter reviews for",
    type=lambda s: tuple(u.strip() for u in s.split(","))
  )
  parser.add_argument(
    "--period",
    help="Time period to analyze (e.g., 10d, 2w, 3m, 24h)",
    required=True,
    type=parse_period,
  )

  return parser.parse_args()


def main():
  args = parse_args()
  print(count_reviews(args.repo, args.users, args.period))


if __name__ == "__main__":
  main()
