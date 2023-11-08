from django import forms
from django.contrib.auth.forms import UserCreationForm
from django.db import connection, IntegrityError
from django.contrib.auth.hashers import make_password
from django.http import JsonResponse
import logging

logger = logging.getLogger(__name__)

class CustomUserCreationForm(UserCreationForm):
    def save(self):
        username : str = self.cleaned_data['username']
        password : str = make_password(self.cleaned_data['password1'])
        try:
            with connection.cursor() as cursor:
                cursor.execute("INSERT INTO users (username, password, image_index) VALUES (%s, %s,%t)", [username, password, 0])
                if cursor.rowcount == 1:
                    logger.debug(f'User {username} registered successfully.')
                    return JsonResponse({'message': 'User registered successfully'}, status=200)
                else:
                    logger.error(f'User registration failed for {username}.')
                    return JsonResponse({'message': 'User registration failed'}, status=500)
        except IntegrityError as e:
            return JsonResponse({'message': 'Username already in use'}, status=500)
        except Exception as e:
            return JsonResponse({'message': 'Database error'}, status=500)