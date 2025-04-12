import pytest


def pytest_addoption(parser: pytest.Parser) -> None:
    parser.addoption(  # type: ignore
        "--host",
        action="store",
        default="http://localhost:8777",
        help="Backend server host URL (default: http://localhost:8777)",
    )
