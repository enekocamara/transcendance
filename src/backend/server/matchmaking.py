from dataclasses import dataclass, field
from collections import deque
import numpy as np
import socket
from .consumers import *
import time
from enum import Enum
from timeit import default_timer as timer

import logging
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

max_num_of_users = 200

def id_gen():
    id = 0
    while True:
        yield id

class LobbyTypes(Enum):
    DUEL = 2,
    GROUP = 4,
    BATTALION = 10

@dataclass(slots=True)
class Player:
    id : int = field(init=False, default_factory=id_gen)
    pos : float = field(init=False, default=0, compare=False, hash=False, repr=False)

@dataclass(slots=True)
class Lobby:
    tick_rate : int = field(init=True, repr=False)
    port : int = field(init=False)#generator 
    type : LobbyTypes = field(init=True)
    udp_socket : socket.socket = field(init=False, repr=False)
    players : np.ndarray = field(init=False)
    in_use : bool = field(default=False)


    def __post_init__(self) -> None:
        self.players = np.empty(self.type, dtype=Player)

    def step():
        pass

@dataclass(slots=True)
class Matchmaking:
    tick_rate : int = field(init=True)
    duelQueue : deque = field(default_factory=deque, repr=False)
    groupQueue : deque = field(default_factory=deque, repr=False)
    battalionQueue : deque = field(default_factory=deque, repr=False)
    current_num_players : int = field(default=0)
    current_num_lobbyes : int = field(default=0)
    num_of_empty_lobbyes : int = field(default=100)
    index_of_last_lobby : int = field(default=0, repr=False)#like a custom memory allocator.
    lobbies : np.ndarray =  field(default_factory=lambda : np.empty(100, dtype=Lobby))
    main_port : int = field(init=False, default=49152)
    main_socket : socket.socket = field(init=False, repr=False)

    def __post_init__(self):
        self.main_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.main_socket.bind(('localhost', self.main_port))

    def step(self):
        for i in range(self.current_num_lobbyes):
            self.lobbies[i].step()
        #handle new connections

def matchmaking():
    mm_handler : Matchmaking = Matchmaking(tick_rate=30)
    tick_rate = 30
    tick_interval = 1 / tick_rate
    while True:
        start_time = timer()
        mm_handler.step()
        end_time = timer()
        elapsed_time = end_time - start_time
        sleep_time = max(0, tick_interval - elapsed_time)
        time.sleep(sleep_time)
        end_time = timer()
        elapsed_time = end_time - start_time
