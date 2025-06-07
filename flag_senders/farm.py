import logging
from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

import requests
from pydantic import BaseModel


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
    sploit: Optional[str]
    team: Optional[str]
    created_time: datetime
    start_waiting_time: Optional[datetime]
    status: FlagStatus
    checksystem_response: Optional[str]


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
    teams: Dict[str, str]


class Config(BaseModel):
    database: DatabaseConfig
    auth: AuthConfig
    ctf: CtfConfig


class BackendClient:
    def __init__(self, host: str, token: str):
        self.host = host.rstrip("/")
        self.protocol = self._determine_protocol()
        if self._check_auth(token):
            self.auth_token = token
        else:
            raise ValueError("Неверный токен")

    def _determine_protocol(self) -> str:
        if self.host.startswith("grpc://"):
            return "grpc"
        return "http"

    def _check_auth(self, password: str) -> bool:
        if self.protocol == "http":
            url = f"{self.host}/api/check_auth"
            headers = {"Content-Type": "application/json", "Accept": "*/*"}
            try:
                payload = {"passwd": password}
                response = requests.post(url, json=payload, headers=headers)
                response.raise_for_status()
                if response.json() == "ok":
                    return True
                else:
                    return False
            except Exception as e:
                logging.error(f"Ошибка аутентификации: {e}")
        else:
            raise ValueError(f"Unsupported protocol: {self.protocol}")
        return False

    def get_config(self) -> Config:
        """
        Получает конфигурацию из бэкенда.

        @return: Конфигурация
        """
        if self.protocol == "http":
            url = f"{self.host}/api/config"
            headers: dict[str, str] = {}
            if self.auth_token:
                headers["X-Authorization"] = self.auth_token
            response = requests.get(url, headers=headers)
            response.raise_for_status()
            return Config(**response.json())
        else:
            raise ValueError(f"Unsupported protocol: {self.protocol}")

    def get_sending_flags(self) -> Optional[List[Flag]]:
        """
        Получает флаги для отправки в журейный сервер.

        @return: Список флагов для отправки
        """
        if self.protocol == "http":
            url = f"{self.host}/api/get_sending_flags"
            headers = {"Content-Type": "application/json"}
            if self.auth_token:
                headers["X-Authorization"] = self.auth_token
            try:
                response = requests.get(url, headers=headers)
                response.raise_for_status()
                flags_data: List[Dict[str, Any]] = response.json()
                return [Flag(**flag_data) for flag_data in flags_data]
            except requests.RequestException as e:
                logging.error(f"Ошибка отправки флагов: {e}")
                return None
        else:
            raise ValueError(f"Unsupported protocol: {self.protocol}")

    def update_all_flags(self, flags: List[Flag]) -> None:
        """
        Обновляет статусы нескольких флагов в бэкенде.

        @param flags: Список флагов для обновления
        """
        if self.protocol == "http":
            url = f"{self.host}/api/update_flags_from_sending"
            headers = {"Content-Type": "application/json"}
            if self.auth_token:
                headers["X-Authorization"] = self.auth_token

            try:
                flags_data = [flag.model_dump(mode="json") for flag in flags]
                response = requests.post(url, json=flags_data, headers=headers)
                response.raise_for_status()
            except requests.RequestException as e:
                logging.error(f"Ошибка обновления флагов: {e}")
                raise
        else:
            raise ValueError(f"Unsupported protocol: {self.protocol}")
