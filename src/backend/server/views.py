from django.http import JsonResponse
from django.db import connection, IntegrityError
from django.views.decorators.csrf import csrf_exempt

from django.contrib.auth.forms import UserCreationForm
from django.contrib.auth import hashers

import logging
import json

logger = logging.getLogger('file')

#take this out for prod
@csrf_exempt
def register_user(request):
    if request.method == 'POST':
        form = UserCreationForm(request.POST)
        if form.is_valid():
            try:
                with connection.cursor() as cursor:
                    cursor.execute("INSERT INTO users (username, password, image_index) VALUES (%s, %s,%t)", [form.cleaned_data['username'], form.password1, 0])
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
        else:
            if 'password2' in form.errors:
                error_message = form.errors['password2'][0]
            return JsonResponse({'message':'Error: incorrect form.'}, status=500)
        data = json.loads(request.body)
        username = data.get('username')
        password = hashers.make_password(data.get('password'))
        logger.debug(f'user {username} tried to register.')
        
    else:
        return JsonResponse({'message': 'Invalid request method'}, status=400)

def login_user(request):
    if request.method == 'POST':
            data = json.loads(request.body)
            username = data.get('username')
            password = data.get('password')
            logger.debug(f'user {username} tried to login.')
            try:
                with connection.cursor() as cursor:
                    cursor.execute("SELECT password FROM users WHERE username = %s", [username])
                    row = cursor.fetchone()
                    if row is not None:
                        if 
                        return JsonResponse({'message': 'Success'}, status=200)
                    else:
                        return JsonResponse({'message':'Error: no such password or username.'}, status=500)
            except Exception as e:
                return JsonResponse({'message': 'Database error'}, status=500)