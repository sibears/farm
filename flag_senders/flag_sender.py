import argparse
import logging
import time
from abc import ABC, abstractmethod
from typing import List

from farm import BackendClient, Config, Flag


class FlagSender(ABC):
	def __init__(self, backend_client: BackendClient):
		self.backend_client = backend_client

	@abstractmethod
	def send_flags(self, config: Config, flags: List[Flag]) -> List[Flag]:
		"""
		Метод, который должен быть реализован в подклассах.
		Отправляет флаги в журейную систему и возвращает ответы.

		@param flags: Список флагов для отправки
		@return: Список ответов от журейки
		"""
		pass

	def run(self) -> None:
		"""
		Запускает периодический процесс отправки флагов.
		"""
		config = self.backend_client.get_config()
		logging.info(
			f"Запуск отправки флагов с журейкой: {config.ctf.protocol.checksys_host}"
		)
		try:
			while True:
				start_time = time.time()
				config = self.backend_client.get_config()
				submit_period = config.ctf.submit_period

				if flags := self.backend_client.get_sending_flags():
					updated_flags = self.send_flags(config, flags)
					self.backend_client.update_all_flags(updated_flags)

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
