import unittest
from unittest.mock import patch
from datetime import datetime, timezone, timedelta
from src.period import Period


class TestPeriod(unittest.TestCase):

  def setUp(self):
    self.start_time = datetime(2024, 1, 1, 12, 00, 59, tzinfo=timezone.utc)
    self.end_time = datetime(2024, 1, 31, 12, 31, 42, tzinfo=timezone.utc)
    self.delta = self.end_time - self.start_time

  def test_init_with_start_and_end(self):
    period = Period(self.start_time, self.end_time)
    self.assertEqual(period.start, self.start_time)
    self.assertEqual(period.end, self.end_time)

  def test_init_with_start_only(self):
    before = datetime.now(timezone.utc)
    period = Period(self.start_time)
    after = datetime.now(timezone.utc)
    self.assertEqual(period.start, self.start_time)
    self.assertIsInstance(period.end, datetime)
    self.assertTrue(period.end >= before)
    self.assertTrue(period.end <= after)

  def test_str_representation(self):
    period = Period(self.start_time, self.end_time)
    expected = "Period from 2024-01-01T12:00:59Z to 2024-01-31T12:31:42Z"
    self.assertEqual(str(period), expected)

  def test_isodatetime_utc_timezone(self):
    dt = datetime(2001, 2, 3, 4, 5, 6, tzinfo=timezone.utc)
    result = Period.isodatetime(dt)
    self.assertEqual(result, "2001-02-03T04:05:06Z")

  def test_isodatetime_different_timezone(self):
    tz_offset = timezone(timedelta(hours=5))
    dt = datetime(2001, 2, 3, 4, 5, 6, tzinfo=tz_offset)
    result = Period.isodatetime(dt)
    self.assertEqual(result, "2001-02-03T04:05:06+05:00")

  @patch('src.period.datetime')
  def test_from_duration(self, mock_datetime):
    mock_now = datetime(2024, 2, 15, 10, 30, 14, tzinfo=timezone.utc)
    mock_datetime.now.return_value = mock_now

    duration = timedelta(days=7)
    period = Period.from_duration(duration)

    expected_start = datetime(2024, 2, 8, 10, 30, 14, tzinfo=timezone.utc)
    expected_end = mock_now

    self.assertEqual(period.start, expected_start)
    self.assertEqual(period.end, expected_end)
    mock_datetime.now.assert_called_once_with(timezone.utc)

  @patch('src.period.datetime')
  def test_contains(self, mock_datetime):
    mock_datetime.now.return_value = self.end_time
    periods = [
      Period(self.start_time),
      Period(self.start_time, self.end_time),
      Period.from_duration(self.delta),
    ]

    base_cases = [
      # (timestamp, expected_result, description)
      (datetime(2024, 1, 15, 12, 0, 0, tzinfo=timezone.utc), True, "within period"),
      (datetime(2023, 12, 31, 12, 0, 0, tzinfo=timezone.utc), False, "before period"),
      (datetime(2024, 2, 1, 12, 0, 0, tzinfo=timezone.utc), False, "after period"),
      (self.start_time, True, "at start boundary"),
      (self.end_time, True, "at end boundary"),
      # Different timezone - same UTC time as 2024-01-15 12:00:00 UTC
      (datetime(2024, 1, 15, 17, 0, 0, tzinfo=timezone(timedelta(hours=5))), True,
       "different timezone within"),
      # Different timezone - before period
      (datetime(2023, 12, 31, 18, 0, 0, tzinfo=timezone(timedelta(hours=6))), False,
       "different timezone before"),
    ]

    test_cases = [(period, timestamp, result, description) for period in periods for
                  timestamp, result, description in base_cases]

    for period, timestamp, expected, description in test_cases:
      with self.subTest(description=description):
        result = period.contains(timestamp)
        self.assertEqual(result, expected)


if __name__ == '__main__':
  unittest.main()
