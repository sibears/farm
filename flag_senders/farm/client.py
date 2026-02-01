from abc import ABC, abstractmethod
from enum import Enum
from pydantic import BaseModel
from datetime import datetime


def parse_datetime(date_string: str) -> datetime:
	if "." in date_string:
		main_part, microseconds = date_string.split(".")
		microseconds = microseconds[:6]  # Усекаем до 6 цифр
		date_string = f"{main_part}.{microseconds}"
	return datetime.strptime(date_string, "%Y-%m-%dT%H:%M:%S.%f")


class FlagStatus(str, Enum):
	QUEUED = "QUEUED"
	WAITING = "WAITING"
	SKIPPED = "SKIPPED"
	ACCEPTED = "ACCEPTED"
	REJECTED = "REJECTED"


class Flag(BaseModel):
	id: int
	flag: str
	sploit: str | None
	team: str | None
	created_time: datetime
	start_waiting_time: datetime | None
	status: FlagStatus
	checksystem_response: str | None


class DatabaseConfig(BaseModel):
	database_url: str


class AuthConfig(BaseModel):
	password: str


class ProtocolConfig(BaseModel):
	protocol: str
	team_token: str
	checksys_host: str
	checksys_port: int


class CtfConfig(BaseModel):
	protocol: ProtocolConfig
	flag_format: str
	flag_lifetime: int
	submit_period: int
	waiting_period: int
	submit_flag_limit: int
	teams: dict[str, str]


class Config(BaseModel):
	database: DatabaseConfig
	auth: AuthConfig
	ctf: CtfConfig

class BackendClient(ABC):
	def __init__(self, host: str, token: str):
		self.host = host.rstrip("/")
		self.token = token

	@abstractmethod
	def get_config(self) -> Config: ...

	@abstractmethod
	def get_sending_flags(self) -> list[Flag] | None: ...

	@abstractmethod
	def update_all_flags(self, flags: list[Flag]) -> None: ...
