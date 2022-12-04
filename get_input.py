#!/usr/bin/python3
import sys
import requests
import os
from dotenv import load_dotenv

load_dotenv()

YEAR = 2022
AOC_URL = f"https://adventofcode.com/{YEAR}/day/%s/input"
AOC_SESSION_ID = os.getenv("AOC_SESSION_ID")

def fetch_inputs(day: str):
    full_url = AOC_URL % (day,)
    response = requests.get(full_url, cookies={"session": AOC_SESSION_ID})
    response.raise_for_status()
    return response.text

def main():
    return fetch_inputs(sys.argv[1])


if __name__ == '__main__':
    main()

