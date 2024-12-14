import socket
import logging

from flag_sender import FlagSender
from farm import Flag, FlagStatus
from typing import List


class RuCtfTcpFlagSender(FlagSender):
    def connect_to_checksystem(self, host: str, port: int) -> socket.socket:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5.0)
        sock.connect((host, port))

        data = sock.recv(4096)
        if not data:
            raise Exception("Нет данных от сервера журейки.")
        if "Enter your flags" not in data.decode("utf-8"):
            raise Exception(f"Неожиданный ответ: {data.decode('utf-8')}")

        return sock

    def send_flags(self, flags: List[Flag]) -> List[Flag]:
        protocol_config = self.config.ctf.protocol

        if not flags:
            logging.debug("Нет флагов для отправки.")
            return []

        try:
            with self.connect_to_checksystem(
                protocol_config.checksys_host, protocol_config.checksys_port
            ) as sock:
                responses: List[str] = []
                try:
                    for flag in flags:
                        sock.sendall(flag.flag.encode("utf-8") + b"\n")
                        logging.info(f"Отправлен флаг: {flag.flag}")

                        if not (data := sock.recv(4096)):
                            break

                        messages = data.decode("utf-8").split("\n")
                        valid_messages = list(filter(None, messages))
                        responses.extend(valid_messages)

                        if len(responses) >= len(flags):
                            break

                except socket.timeout:
                    logging.info("Таймаут ожидания ответов от журейки.")

                # Обработка ответов и обновление статусов
                flags_to_update = flags[: len(responses)]
                for flag, response in zip(flags_to_update, responses):
                    if "Accepted" in response:
                        flag.status = FlagStatus.ACCEPTED
                    elif "Denied" in response:
                        flag.status = FlagStatus.REJECTED
                    flag.checksystem_response = response

                    if "Accepted" in response or "Denied" in response:
                        logging.info(f"Ответ от журейки: {response}")
                    else:
                        logging.warning(f"Неожиданный ответ: {response}")

                return flags_to_update

        except socket.error as e:
            logging.error(f"Ошибка сокета: {e}")
        except Exception as e:
            logging.error(f"Произошла ошибка при отправке флагов: {e}")

        return []
