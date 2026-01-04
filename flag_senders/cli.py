import logging
import time
import argparse

from farm import BackendClient
from protocols import get_protocol_cls, BaseProtocol

def mainloop(backend: BackendClient, protocol: BaseProtocol) -> None:
	"""
	Запускает периодический процесс отправки флагов.
	"""
	config = backend.get_config()
	logging.info(
		f"Запуск отправки флагов с журейкой: {config.ctf.protocol.checksys_host}"
	)
	try:
		while True:
			start_time = time.time()
			config = backend.get_config()
			submit_period = config.ctf.submit_period

			if flags := backend.get_sending_flags():
				updated_flags = protocol.send_flags(config, flags)
				backend.update_all_flags(updated_flags)

			elapsed = time.time() - start_time
			sleep_time = max(0, submit_period - elapsed)
			logging.info(
				f"Ожидание {sleep_time:.2f} секунд перед следующей отправкой [период отправки {submit_period} секунд]."
			)
			time.sleep(sleep_time)
	except KeyboardInterrupt:
		logging.info("Остановка отправки флагов пользователем.")
	except Exception as e:
		logging.error(f"Произошла ошибка: {e}")

def parse_args() -> argparse.Namespace:
	parser = argparse.ArgumentParser(description="Flag Sender Client")
	parser.add_argument(
		"--host",
		required=True,
		help="Backend server host URL (e.g., http://localhost:8000)",
	)
	parser.add_argument("--token", required=True, help="Пароль от фермы")

	return parser.parse_args()

def main() -> None:
	logging.basicConfig(
		level=logging.INFO,
		format="%(asctime)s [%(levelname)s] %(message)s",
		handlers=[logging.StreamHandler()],
	)

	args = parse_args()
	backend_client = BackendClient(args.host, args.token)
	config = backend_client.get_config()

	protocol = config.ctf.protocol.protocol

	if (sender_cls := get_protocol_cls(protocol)) is None:
		raise ValueError(f"Unsupported farm protocol: {protocol}")
	sender = sender_cls()
	mainloop(backend_client, sender)


if __name__ == "__main__":
	main()
