FROM python:3.12-slim
WORKDIR /tests
RUN pip install --no-cache-dir -U pip
COPY requirements.txt /tests/requirements.txt
RUN pip install --no-cache-dir -r requirements.txt
COPY tests /tests/tests
