from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_POST
from django.contrib.auth.decorators import login_required
from django.http import JsonResponse


@csrf_exempt
@require_POST
@login_required
def game(request):
    return JsonResponse({'messaeg':'success you are gaming'}, status=200)