import logging
import requests
from typing import List

from farm import Config, Flag, FlagStatus
from flag_sender import FlagSender


class SibirCtfHttpFlagSender(FlagSender):
    def send_flags(self, config: Config, flags: List[Flag]) -> List[Flag]:
        protocol_config = config.ctf.protocol
        
        if not flags:
            logging.debug("Нет флагов для отправки.")
            return []

        flags_to_update: List[Flag] = []
        for flag in flags:
            params = {
                "teamid": protocol_config.team_token,
                "flag": flag.flag
            }
            url = f"http://{protocol_config.checksys_host}:{protocol_config.checksys_port}/flag"            
            try:
                response = requests.get(url, params=params)
                
                if response.status_code == 200:
                    flag.status = FlagStatus.ACCEPTED
                    logging.info(f"Флаг принят: {flag.flag} - {response.text}")
                elif response.status_code == 403:
                    flag.status = FlagStatus.REJECTED
                    logging.info(f"Флаг отклонен: {flag.flag} - {response.text}")
                elif response.status_code == 400:
                    flag.status = FlagStatus.REJECTED
                    logging.warning(f"Некорректный запрос: {flag.flag} - {response.text}")
                else:
                    flag.status = FlagStatus.QUEUED
                    logging.warning(f"Неизвестный ответ: {response.status_code}, {response.text}")
                
                flag.checksystem_response = response.text
                flags_to_update.append(flag)
            
            except requests.RequestException as e:
                logging.error(f"Ошибка при отправке флага {flag.flag}: {e}")
                flag.status = FlagStatus.QUEUED
                flags_to_update.append(flag)
        
        return flags_to_update