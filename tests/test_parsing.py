import unittest
import argparse
from datetime import timedelta
from src.parsing import parse_period


class TestParsing(unittest.TestCase):

  def test_parse_period_hours(self):
    result = parse_period("24h")
    expected = timedelta(hours=24)
    self.assertEqual(result, expected)

  def test_parse_period_days(self):
    result = parse_period("7d")
    expected = timedelta(days=7)
    self.assertEqual(result, expected)

  def test_parse_period_weeks(self):
    result = parse_period("2w")
    expected = timedelta(weeks=2)
    self.assertEqual(result, expected)

  def test_parse_period_months(self):
    result = parse_period("3m")
    expected = timedelta(days=90)  # 3 * 30 days
    self.assertEqual(result, expected)

  def test_parse_period_case_insensitive(self):
    test_cases = [
      ("10H", timedelta(hours=10)),
      ("5D", timedelta(days=5)),
      ("1W", timedelta(weeks=1)),
      ("2M", timedelta(days=60)),
    ]

    for input_str, expected in test_cases:
      with self.subTest(input=input_str):
        result = parse_period(input_str)
        self.assertEqual(result, expected)

  def test_parse_period_zero_value(self):
    result = parse_period("0h")
    expected = timedelta(hours=0)
    self.assertEqual(result, expected)

  def test_parse_period_large_numbers(self):
    result = parse_period("365d")
    expected = timedelta(days=365)
    self.assertEqual(result, expected)

  def test_parse_period_invalid_format(self):
    invalid_inputs = [
      "",  # empty string
      "10",  # no unit
      "m"  # no number
      "d10",  # unit before number
      "10x",  # invalid unit
      "ten-d",  # non-numeric value
      "10 d",  # space
      "10dd",  # multiple units
      "-1h"  # negative value
      "4.2d"  # decimal value
      "pi"
      "gibberish"
    ]

    for invalid_input in invalid_inputs:
      with self.subTest(input=invalid_input):
        with self.assertRaises(argparse.ArgumentTypeError):
          parse_period(invalid_input)


if __name__ == '__main__':
  unittest.main()
