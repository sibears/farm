FROM python:3.13-alpine
LABEL authors="sibears"

WORKDIR /app
COPY ./pseudo_saar_tcp.py ./main.py

EXPOSE 31337
CMD ["python3", "main.py"]
