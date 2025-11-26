import logging
from typing import List

import requests

from farm import Config, Flag, FlagStatus
from flag_sender import FlagSender


class RuCtfHttpFlagSender(FlagSender):
	RESPONSES = {
		FlagStatus.QUEUED: [
			"timeout",
			"game not started",
			"try again later",
			"game over",
			"is not up",
			"no such flag",
		],
		FlagStatus.ACCEPTED: ["accepted", "congrat"],
		FlagStatus.REJECTED: [
			"bad",
			"wrong",
			"expired",
			"unknown",
			"your own",
			"too old",
			"not in database",
			"already submitted",
			"invalid flag",
			"denied",
			"stolen",
		],
	}

	def determine_flag_status(self, response_text: str) -> tuple[FlagStatus, str]:
		response_lower = response_text.lower()
		for status, substrings in self.RESPONSES.items():
			if any(s in response_lower for s in substrings):
				return status, response_text

		logging.warning(
			f"Неизвестный ответ от журейки (флаг будет отправлен повторно): {response_text}"
		)
		return FlagStatus.QUEUED, response_text

	def send_flags(self, config: Config, flags: List[Flag]) -> List[Flag]:
		protocol_config = config.ctf.protocol

		if not flags:
			logging.debug("Нет флагов для отправки.")
			return []

		try:
			flags_to_submit = [flag.flag for flag in flags]
			response = requests.put(
				f"http://{protocol_config.checksys_host}:{protocol_config.checksys_port}/flags",
				headers={"X-Team-Token": protocol_config.team_token},
				json=flags_to_submit,
				timeout=config.ctf.submit_period,
			)

			if not response.ok:
				logging.error(
					f"Ошибка HTTP при отправке флагов: {response.status_code} {response.text}"
				)
				return []

			response_data = response.json()
			flags_to_update: list[Flag] = []

			for i, item in enumerate(response_data):
				if i >= len(flags):
					break

				flag = flags[i]
				response_msg = item.get("msg", "").strip()
				response_msg = response_msg.replace(f"[{item.get('flag', '')}] ", "")

				status, message = self.determine_flag_status(response_msg)
				flag.status = status
				flag.checksystem_response = message

				if status == FlagStatus.ACCEPTED:
					logging.info(f"Флаг принят: {flag.flag} - {message}")
				elif status == FlagStatus.REJECTED:
					logging.info(f"Флаг отклонен: {flag.flag} - {message}")
				else:
					logging.info(f"Флаг в очереди: {flag.flag} - {message}")

				flags_to_update.append(flag)

			return flags_to_update

		except requests.exceptions.Timeout:
			logging.error("Таймаут при отправке флагов в журейку")
		except requests.exceptions.RequestException as e:
			logging.error(f"Ошибка HTTP-запроса: {e}")
		except Exception as e:
			logging.error(f"Произошла ошибка при отправке флагов: {e}")

		return []
