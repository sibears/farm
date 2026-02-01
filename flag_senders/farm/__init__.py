from errors import CliError
from .client import Config, Flag, FlagStatus, BackendClient
from .http_client import HttpBackendClient


def get_client_cls(host: str) -> type[BackendClient]:
	if host.startswith("http"):
		return HttpBackendClient
	else:
		raise CliError(f"Unsupported protocol for host: {host}")

__all__ = ["Config", "Flag", "FlagStatus", "HttpBackendClient"]
