import logging
import socket
from typing import List

from farm import Config, Flag, FlagStatus
from flag_sender import FlagSender


class SaarTcpFlagSender(FlagSender):
	def connect_to_checksystem(self, host: str, port: int) -> socket.socket:
		sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
		sock.settimeout(5.0)
		sock.connect((host, port))
		return sock

	def send_flags(self, config: Config, flags: List[Flag]) -> List[Flag]:
		protocol_config = config.ctf.protocol

		if not flags:
			logging.debug("No flags to submit.")
			return []

		try:
			with self.connect_to_checksystem(
				protocol_config.checksys_host, protocol_config.checksys_port
			) as sock:
				flags_to_update = []
				try:
					for flag in flags:
						sock.sendall(flag.flag.encode("utf-8") + b"\n")
						logging.info(f"Submitted flag: {flag.flag}")

						if not (data := sock.recv(4096)):
							break

						response = data.decode("utf-8").strip()
						flag.checksystem_response = response
						flags_to_update.append(flag)

						if response.startswith("[OK]"):
							flag.status = FlagStatus.ACCEPTED
							logging.info(f"Flag accepted: {response}")
						elif response.startswith("[ERR]"):
							flag.status = FlagStatus.REJECTED
							logging.info(f"Flag rejected: {response}")
						elif response.startswith("[OFFLINE]"):
							flag.status = FlagStatus.QUEUED
							logging.warning(f"Submission system offline: {response}")
						else:
							flag.status = FlagStatus.REJECTED
							logging.warning(f"Unexpected response: {response}")

				except socket.timeout:
					logging.info("Timeout waiting for responses from checksystem.")

				return flags_to_update

		except socket.error as e:
			logging.error(f"Socket error: {e}")
		except Exception as e:
			logging.error(f"Error occurred while submitting flags: {e}")

		return []
