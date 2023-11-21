import json
from channels.generic.websocket import AsyncWebsocketConsumer
#from .models import *

#from .models import MatchmakingQueue

import logging

logger = logging.getLogger(__name__)

class MatchmakingConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        print('fucking connect already fatass')
        logger.debug('SOMEBODY connected using sockets')
        await self.accept()

    async def disconnect(self, close_code):
        # Get the user associated with the WebSocket connection
        user = self.scope["user"]

        # Remove the user from the matchmaking queue
    #    await self.remove_from_matchmaking_queue(user)

    async def notify_match(self, event):
        message = event['message']
        await self.send(text_data=json.dumps({'message': message}))

   # async def remove_from_matchmaking_queue(self, user):
   #     try:
   #         # Retrieve the player profile associated with the user
   #         player_profile = CustomUser.objects.get(user=user)
#
   #         # Remove the player from the matchmaking queue
   #         try:
   #             matchmaking_queue = MatchmakingQueue.objects.get(players=player_profile)
   #             matchmaking_queue.players.remove(player_profile)
   #         except MatchmakingQueue.DoesNotExist:
   #             pass  # Handle the case where the player is not in the queue
#
   #     except PlayerProfile.DoesNotExist:
   #         pass  # Handle the case where the player profile does not exist
