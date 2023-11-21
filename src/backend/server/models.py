from django.contrib.auth.models import AbstractUser
from django.db import models

#from ..members.models import *

from ..members.models import CustomUser


class MatchmakingQueue(models.Model):
    players = models.ManyToManyField(CustomUser)