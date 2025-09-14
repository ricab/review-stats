import argparse
import re
from datetime import timedelta


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

  assert False, "unreachable: accepted but unhandled unit"
