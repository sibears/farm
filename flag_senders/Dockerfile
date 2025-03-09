FROM python:3.13-slim as builder
LABEL authors="sibears"
COPY --link --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/

WORKDIR /app

ENV UV_COMPILE_BYTECODE=1

RUN --mount=type=cache,target=/root/.cache/uv \
    --mount=type=bind,source=uv.lock,target=uv.lock \
    --mount=type=bind,source=pyproject.toml,target=pyproject.toml \
    uv sync --frozen --no-install-project

COPY . .

RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen

ENV PATH="/app/.venv/bin:$PATH"
ENV HOST="http://back:8000"
ENV TOKEN="sibears1cool"
ENV PROTOCOL="ructf_http"

CMD ["sh", "-c", "uv run cli.py --host=${HOST} --token=${TOKEN} --protocol=${PROTOCOL}"]