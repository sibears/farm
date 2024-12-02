#!/usr/bin/env python3

import argparse

from farm import BackendClient


def parse_args():
    parser = argparse.ArgumentParser(description="Backend Server Client")
    parser.add_argument(
        "--host",
        required=True,
        help="Backend server host URL (e.g., http://localhost:8000)",
    )
    subparsers = parser.add_subparsers(dest="command", required=True, help="Commands")

    # Subparser for get_config
    subparsers.add_parser("get_config", help="Fetch configuration from backend")

    # Subparser for send_flags
    subparsers.add_parser("get_sending_flags", help="Get flags for sending")

    return parser.parse_args()


def main():
    args = parse_args()
    client = BackendClient(args.host)

    if args.command == "get_config":
        config = client.get_config()
        if config:
            print("Config:", config)
    elif args.command == "get_sending_flags":
        response = client.get_sending_flags()
        if response:
            print("Send Flags Response:", response)


if __name__ == "__main__":
    main()
