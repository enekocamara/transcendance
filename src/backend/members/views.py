from django.contrib.auth import  authenticate, login
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_POST
from django.http import JsonResponse

from .forms import CustomUserCreationForm

import logging
import json

logger = logging.getLogger(__name__)

@csrf_exempt
@require_POST
def register_user(request):
    data = json.loads(request.body)
    form = CustomUserCreationForm(data)
    logger.debug('IM HERE')
    if form.is_valid():
        form.save()
        return JsonResponse({'message': 'Success'}, status=200)
    else:
        return JsonResponse({'errors':form.errors.as_ul()}, status=500)
#take this out for prod


@csrf_exempt
@require_POST
def login_user(request):
    logger.debug('Someone tried to login')
    data = json.loads(request.body)
    user = authenticate(request=request, username=data['username'], password=data['password'])
    if user is not None:
        logger.debug(f'User: {user.get_username()} has successfully login.')
        login(request, user)
        return JsonResponse({'message':'Login successful', 'session_id': request.session.session_key}, status=200)
    else:
        return JsonResponse({'message':'Failed to login'}, status=401)
