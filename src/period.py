from datetime import datetime, timezone, timedelta
from typing import Optional


class Period:
  def __init__(self, start: datetime, end: Optional[datetime] = None):
    self.start = start
    self.end = end or datetime.now(timezone.utc)

  @classmethod
  def from_duration(cls, duration: timedelta) -> 'Period':
    """Create a period spanning from now back by duration (e.g., last 2 weeks)"""
    now = datetime.now(timezone.utc)
    return cls(now - duration, now)

  @staticmethod
  def isodatetime(dt: datetime) -> str:
    return dt.isoformat(timespec='seconds').replace('+00:00', 'Z')

  def __str__(self) -> str:
    """Pretty print the time period"""
    start_iso = self.isodatetime(self.start)
    end_iso = self.isodatetime(self.end)
    return f"Period from {start_iso} to {end_iso}"

  def contains(self, timestamp: datetime) -> bool:
    """Check if the timestamp falls within this period"""
    return self.start <= timestamp <= self.end
