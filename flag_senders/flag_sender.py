from abc import ABC, abstractmethod
import argparse
import time
import logging
from farm import BackendClient, Flag
from typing import List


class FlagSender(ABC):
    def __init__(self, backend_url: str):
        self.backend_client = BackendClient(backend_url)
        self.config = self.backend_client.get_config()
        self.submit_period = self.config.ctf.submit_period

    @abstractmethod
    def send_flags(self, flags: List[Flag]) -> List[Flag]:
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
        logging.info(f"Запуск отправки флагов каждые {self.submit_period} секунд.")
        try:
            while True:
                start_time = time.time()
                logging.debug("Получение флагов для отправки.")

                if flags := self.backend_client.get_sending_flags():
                    updated_flags = self.send_flags(flags)
                    self.backend_client.update_all_flags(updated_flags)

                elapsed = time.time() - start_time
                sleep_time = max(0, self.submit_period - elapsed)
                logging.info(
                    f"Ожидание {sleep_time:.2f} секунд перед следующей отправкой."
                )
                time.sleep(sleep_time)
        except KeyboardInterrupt:
            logging.info("Остановка отправки флагов пользователем.")
        except Exception as e:
            logging.error(f"Произошла ошибка: {e}")


def parse_args():
    parser = argparse.ArgumentParser(description="Flag Sender Client")
    parser.add_argument(
        "--host",
        required=True,
        help="Backend server host URL (e.g., http://localhost:8000)",
    )
    parser.add_argument(
        "--protocol",
        required=True,
        choices=["ructf_tcp"],
        help="Protocol to use for sending flags",
    )
    return parser.parse_args()
