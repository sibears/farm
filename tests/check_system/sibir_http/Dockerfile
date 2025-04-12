FROM python:3.13-slim
LABEL authors="Jl4cTuk"

WORKDIR /app
RUN pip install flask
COPY ./pseudo_sibir_http.py ./main.py

EXPOSE 8780
CMD ["python3", "main.py"]
