from .base_protocol import BaseProtocol
from .ructf_http import RuCtfHttpFlagSender
from .ructf_tcp import RuCtfTcpFlagSender
from .sibir_http import SibirCtfHttpFlagSender
from .saar_tcp import SaarTcpFlagSender

proto_list = [
	RuCtfHttpFlagSender,
	RuCtfTcpFlagSender,
	SibirCtfHttpFlagSender,
	SaarTcpFlagSender,
]

def get_protocol_cls(protocol: str) -> type[BaseProtocol] | None:
	if protocol not in [proto.protocol for proto in proto_list]:
		return None
	for proto in proto_list:
		if proto.protocol == protocol:
			return proto

__all__ = [
	"BaseProtocol",
	"RuCtfHttpFlagSender",
	"SibirCtfHttpFlagSender",
	"RuCtfTcpFlagSender",
	"SaarTcpFlagSender",
]
