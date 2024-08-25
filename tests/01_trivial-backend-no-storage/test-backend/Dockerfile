FROM python:3.10.8-slim

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

ENV PYTHONUNBUFFERED 1

RUN pip install poetry==1.8.3

COPY ./pyproject.toml .

RUN POETRY_VIRTUALENVS_CREATE=false poetry install -n --no-root

EXPOSE 8000

COPY . .
