from flag_sender import parse_args
from protocols.ructf_tcp import RuCtfTcpFlagSender
import logging


def main():
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(message)s",
        handlers=[logging.StreamHandler()],
    )

    args = parse_args()

    if args.protocol == "ructf_tcp":
        sender = RuCtfTcpFlagSender(args.host)
    else:
        raise ValueError(f"Unsupported farm protocol: {args.protocol}")

    sender.run()


if __name__ == "__main__":
    main()
