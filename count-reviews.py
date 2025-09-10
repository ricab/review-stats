#!/usr/bin/env python3

import argparse
import re
from datetime import datetime, timedelta, timezone
from typing import Optional


class Period:
  def __init__(self, start: datetime, end: Optional[datetime] = None):
    self.start = start
    self.end = end or datetime.now(timezone.utc)

  def __str__(self) -> str:
    """Pretty print the time period"""
    start_iso = self.isodatetime(self.start)
    end_iso = self.isodatetime(self.end)
    return f"Period from {start_iso} to {end_iso}"

  @staticmethod
  def isodatetime(dt: datetime) -> str:
    return dt.isoformat(timespec='seconds').replace('+00:00', 'Z')


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


def count_reviews(repo, users, period):
  return {"repository": repo, "users": users, "period": period}


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
