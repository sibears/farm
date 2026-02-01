import logging
import requests
from typing import Any

from .client import BackendClient, Config, Flag


class HttpBackendClient(BackendClient):
	@staticmethod
	def check_auth(host: str, password: str) -> bool:
		url = f"{host}/api/check_auth"
		headers = {"Content-Type": "application/json", "Accept": "*/*"}
		try:
			payload = {"passwd": password}
			response = requests.post(url, json=payload, headers=headers, timeout=5)
			response.raise_for_status()
			if response.json() == "ok":
				return True
			else:
				return False
		except Exception as e:
			logging.error(f"Ошибка аутентификации: {e}")
		return False

	def get_config(self) -> Config:
		"""
		Получает конфигурацию из бэкенда.

		@return: Конфигурация
		"""
		url = f"{self.host}/api/config"
		headers: dict[str, str] = {}
		if self.token:
			headers["X-Authorization"] = self.token
		response = requests.get(url, headers=headers, timeout=5)
		response.raise_for_status()
		return Config(**response.json())

	def get_sending_flags(self) -> list[Flag] | None:
		"""
		Получает флаги для отправки в журейный сервер.

		@return: Список флагов для отправки
		"""
		url = f"{self.host}/api/get_sending_flags"
		headers = {"Content-Type": "application/json"}
		if self.token:
			headers["X-Authorization"] = self.token
		try:
			response = requests.get(url, headers=headers, timeout=5)
			response.raise_for_status()
			flags_data: list[dict[str, Any]] = response.json()
			return [Flag(**flag_data) for flag_data in flags_data]
		except requests.RequestException as e:
			logging.error(f"Ошибка отправки флагов: {e}")
			return None

	def update_all_flags(self, flags: list[Flag]) -> None:
		"""
		Обновляет статусы нескольких флагов в бэкенде.

		@param flags: Список флагов для обновления
		"""
		url = f"{self.host}/api/update_flags_from_sending"
		headers = {"Content-Type": "application/json"}
		if self.token:
			headers["X-Authorization"] = self.token

		try:
			flags_data = [flag.model_dump(mode="json") for flag in flags]
			response = requests.post(url, json=flags_data, headers=headers, timeout=5)
			response.raise_for_status()
		except requests.RequestException as e:
			logging.error(f"Ошибка обновления флагов: {e}")
