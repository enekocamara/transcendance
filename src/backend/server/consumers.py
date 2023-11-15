import json
from channels.generic.websocket import AsyncWebsocketConsumer
from dataclasses import dataclass, field
from collections import deque
import numpy as np
import socket

from enum import Enum

max_num_of_users = 200

class LobbyTypes(Enum):
    DUEL = 2,
    GROUP = 4,
    BATTALION = 10

@dataclass
class Player:
    id : int #generator?
    pos : float = field(default=0)

@dataclass
class Lobby:
    port : int #generator
    type : LobbyTypes
    players : np.ndarray = np.empty(0, dtype=Player)
    udp_socket : socket.socket #generator


class ServerConsumer(AsyncWebsocketConsumer):
    async def disconnect(self):
        await self.disconnect()
    async def receive(self, text_data):
        data = json.loads(text_data)
    async def game_loop(self):
        pass
    async def connect(self):
        await self.connect()
        await self.game_loop()
