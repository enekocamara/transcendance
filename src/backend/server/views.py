from django.http import JsonResponse
from django.db import connection, IntegrityError
from django.views.decorators.csrf import csrf_exempt
from .authBackend import authenticate

from .forms import CustomUserCreationForm

import logging
import json

logger = logging.getLogger(__name__)

#take this out for prod
@csrf_exempt
def register_user(request):
    if request.method == 'POST':
        data = json.loads(request.body)
        form = CustomUserCreationForm(data)
        if form.is_valid():
            return form.save()
        else:
            return JsonResponse({'message':form.errors.as_ul()}, status=500)
    else:
        return JsonResponse({'message': 'Invalid request method'}, status=400)

@csrf_exempt
def login_user(request):
    if request.method == 'POST':
        data = json.loads(request.body)
        logger.debug(data)
        username : str = data.get('username')
        password : str = data.get('password')
        if authenticate(username, password):
            return JsonResponse({'message': 'Login success'}, status=200)
        else:
            return JsonResponse({'message': 'Username or password does not exist'}, status=500)            
        #    
        #    
        #    
        #    
        #    logger.debug(f'user {username} tried to login.')
        #    try:
        #        with connection.cursor() as cursor:
        #            cursor.execute("SELECT password FROM users WHERE username = %s", [username])
        #            row = cursor.fetchone()
        #            if row is not None:
        #                #if 
        #                return JsonResponse({'message': 'Success'}, status=200)
        #            else:
        #                return JsonResponse({'message':'No such password or username.'}, status=500)
        #    except Exception as e:
        #        return JsonResponse({'message': 'Database error'}, status=500)
            
def game(request):
    pass