from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_POST
from django.contrib.auth.decorators import login_required
from django.http import JsonResponse

import logging

logger = logging.getLogger(__name__)

#@login_required
@csrf_exempt
@require_POST
def matchmaking(request):
    logger.debug('somebody wants to play')
    return JsonResponse({'messaeg':'Accepted','Matchmaking Port':49152}, status=200)