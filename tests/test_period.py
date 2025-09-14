import unittest
from datetime import datetime, timezone, timedelta
from src.period import Period


class TestPeriod(unittest.TestCase):

    def setUp(self):
        self.start_time = datetime(2024, 1, 1, 12, 00, 59, tzinfo=timezone.utc)
        self.end_time = datetime(2024, 1, 31, 12, 31, 42, tzinfo=timezone.utc)

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

if __name__ == '__main__':
    unittest.main()
