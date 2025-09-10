#!/usr/bin/env python3

import argparse
import re
from datetime import timedelta

from src.period import Period
from src.review_inspector import count_reviews


def parse_period(period_str):
  """Parse a period string like '10d', '2w', '3m', '24h' into timedelta"""
  match = re.match(r'^(\d+)([hdwm])$', period_str.lower())
  if not match:
    raise argparse.ArgumentTypeError(
      f"Invalid period format: {period_str}. Use format like 10d, 2w, 3m, 24h")

  value, unit = match.groups()
  value = int(value)

  if unit == 'h':
    return timedelta(hours=value)
  elif unit == 'd':
    return timedelta(days=value)
  elif unit == 'w':
    return timedelta(weeks=value)
  elif unit == 'm':
    return timedelta(days=value * 30)  # Approximate months as 30 days


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
