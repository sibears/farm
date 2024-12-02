from enum import Enum
import requests
import sys

from pydantic import BaseModel
from typing import Dict, Optional
from datetime import datetime


def parse_datetime(date_string: str) -> datetime:
    if "." in date_string:
        main_part, microseconds = date_string.split(".")
        microseconds = microseconds[:6]  # Усукаем до 6 цифр
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
    def __init__(self, host: str):
        self.host = host.rstrip("/")

    def get_config(self) -> Config:
        url = f"{self.host}/api/config"
        response = requests.get(url)
        response.raise_for_status()
        return Config(**response.json())

    def get_sending_flags(self):
        url = f"{self.host}/api/get_sending_flags"
        headers = {"Content-Type": "application/json"}
        try:
            response = requests.get(url, headers=headers)
            response.raise_for_status()
            return response.json()
        except requests.RequestException as e:
            print(f"Error sending flags: {e}", file=sys.stderr)
            return None
