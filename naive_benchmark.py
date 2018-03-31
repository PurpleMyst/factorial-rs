#!/usr/bin/env python3
import random
import time

try:
    from factorial import factorial
except ImportError:
    import sys
    sys.exit("Please run `./build.sh` before bench-marking.")


def main():
    n = random.randint(20000, 30000)
    print("Calculating the factorial for", n)
    start = time.monotonic()
    res = factorial(n)
    end = time.monotonic()
    print(f"Running took {end - start} seconds.")

    if input("Type y if you want to see the result.").lower().startswith("y"):
        print(res)


if __name__ == "__main__":
    main()
