from abc import ABC, abstractmethod
from farm import Flag, Config


class BaseProtocol(ABC):
	protocol = ""

	@abstractmethod
	def send_flags(self, config: Config, flags: list[Flag]) -> list[Flag]: ...
