from channels.generic.websocket import AsyncWebsocketConsumer
from dataclasses import dataclass, field
from collections import deque
import numpy as np
import socket
from .consumers import *


@dataclass
class WebsocketManager:
    duelQueue : deque
    groupQueue : deque
    battalionQueue : deque
    current_num_players : int = field(default=0)
    num_of_empty_lobbyes : int = field(default=100)
    index_of_last_lobby : int = field(default=0)#like a custom memory allocator.
    lobbies : np.ndarray = np.empty(100, dtype=Lobby)
    websockets : socket.socket
