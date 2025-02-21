from flag_sender import parse_args
from protocols.ructf_tcp import RuCtfTcpFlagSender
from protocols.ructf_http import RuCtfHttpFlagSender
import logging


def main():
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(message)s",
        handlers=[logging.StreamHandler()],
    )

    args = parse_args()

    if args.protocol == "ructf_tcp":
        sender = RuCtfTcpFlagSender(args.host, args.token)
    elif args.protocol == "ructf_http":
        sender = RuCtfHttpFlagSender(args.host, args.token)
    else:
        raise ValueError(f"Unsupported farm protocol: {args.protocol}")

    sender.run()


if __name__ == "__main__":
    main()
