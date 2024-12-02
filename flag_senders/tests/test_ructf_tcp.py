import pytest
import socket
from farm import BackendClient, Flag, FlagStatus, parse_datetime


@pytest.fixture
def client(request: pytest.FixtureRequest) -> BackendClient:
    host: str = request.config.getoption("--host")  # type: ignore
    client = BackendClient(host)
    return client


def test_send_flags(client: BackendClient) -> None:
    test_flags = [
        Flag(
            id=1,
            flag="R0ZIQFZGPD8MBP2WVZSKLQL52FA6A29=",
            sploit="brute.py",
            team="Second",
            created_time=parse_datetime("2024-12-01T18:26:13.150571"),
            start_waiting_time=parse_datetime("2024-12-01T18:26:16.105748056"),
            status=FlagStatus.WAITING,
            checksystem_response=None,
        ),
        Flag(
            id=2,
            flag="O20QXT3EIAE6BXX4NSTO12PN78VY0ZW=",
            sploit="brute.py",
            team="Second",
            created_time=parse_datetime("2024-12-01T18:26:13.150689"),
            start_waiting_time=parse_datetime("2024-12-01T18:26:16.105748998"),
            status=FlagStatus.WAITING,
            checksystem_response=None,
        ),
    ]

    config = client.get_config()

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect(
            (config.ctf.protocol.checksys_host, config.ctf.protocol.checksys_port)
        )
        sock.settimeout(2.0)
        data = sock.recv(4096)
        if not data:
            raise Exception("No data received")
        data = data.decode("utf-8")
        assert "Enter your flags" in data, f"Неожиданный ответ от сервера: {data}"

        for flag in test_flags:
            sock.sendall(flag.flag.encode("utf-8") + b"\n")

        # Получение и проверка ответов
        responses: list[str] = []
        try:
            while True:
                data = sock.recv(4096)
                if not data:
                    break
                messages = data.decode("utf-8").split("\n")
                for msg in messages:
                    if msg:
                        responses.append(msg)
        except socket.timeout:
            pass

    assert len(responses) >= 2, "Недостаточно полученных ответов"
    for response in responses[:2]:
        assert (
            "Accepted" in response or "Denied" in response
        ), f"Неожиданный ответ: {response}"
