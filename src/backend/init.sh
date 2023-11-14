#!/bin/bash

./wait-for-it.sh db:5432 --timeout=30 --

python manage.py makemigrations members

python manage.py migrate

python manage.py createsuperuser --username $DJANGO_SUPERUSER_USERNAME --email $DJANGO_SUPERUSER_EMAIL --noinput

python manage.py runserver  0.0.0.0:5000