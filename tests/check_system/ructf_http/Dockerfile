FROM python:3.13-slim
LABEL authors="stormtrooperroman"

WORKDIR /app
RUN pip install flask
COPY ./pseudo_http.py ./main.py

EXPOSE 8779
CMD ["python3", "main.py"]
