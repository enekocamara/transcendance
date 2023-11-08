from typing import Any
from django.contrib.auth.backends import BaseBackend
from django.contrib.auth.hashers import check_password
from django.db import connection, IntegrityError

import logging

logger = logging.getLogger(__name__)

def authenticate(self, username : str = None, password : str = None) -> bool:
    if username is None or password is None:
        return False
    logger.debug('BOTH exist')
    try:
        with connection.cursor() as cursor:
            logger.debug('YESSSSS')
            cursor.execute("SELECT password FROM users WHERE username = %s", [username])
            row = cursor.fetchone()
            if row is not None:
                return check_password(password=password, encoded=row['password'])
            else:
                return False
    except Exception as _:
        return False