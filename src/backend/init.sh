#!/bin/bash



./wait-for-it.sh db:5432 --timeout=30 --

python manage.py makemigrations members

python manage.py migrate

python manage.py createsuperuser --username $DJANGO_SUPERUSER_USERNAME --email $DJANGO_SUPERUSER_EMAIL --noinput

echo 'lauching...'

pip install 'uvicorn[standard]'

export DJANGO_SETTINGS_MODULE=server.settings

uvicorn server.asgi:application --reload --port 5000

#python manage.py runserver  0.0.0.0:5000