import logging

from farm import BackendClient
from flag_sender import parse_args
from protocols.ructf_http import RuCtfHttpFlagSender
from protocols.ructf_tcp import RuCtfTcpFlagSender
from protocols.sibir_http import SibirCtfHttpFlagSender


def main():
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(message)s",
        handlers=[logging.StreamHandler()],
    )

    args = parse_args()
    backend_client = BackendClient(args.host, args.token)
    config = backend_client.get_config()

    protocol = config.ctf.protocol.protocol

    if protocol == "ructf_tcp":
        sender = RuCtfTcpFlagSender(backend_client)
    elif protocol == "ructf_http":
        sender = RuCtfHttpFlagSender(backend_client)
    elif protocol == "sibir_http":
        sender = SibirCtfHttpFlagSender(backend_client)
    else:
        raise ValueError(f"Unsupported farm protocol: {args.protocol}")

    sender.run()


if __name__ == "__main__":
    main()
