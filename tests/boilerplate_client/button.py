from abc import ABCMeta, abstractmethod
import socket
import time

WAIT_TIME_SEC = 0.5

class Button(metaclass=ABCMeta):
    @abstractmethod
    def right_click(self):
        ...

    @abstractmethod
    def left_click(self):
        ...

    @abstractmethod
    def both_click(self):
        ...

    @abstractmethod
    def close(self):
        ...


class ButtonFake(Button):
    def right_click(self):
        pass

    def left_click(self):
        pass

    def both_click(self):
        pass

    def close(self):
        pass


class ButtonTCP(Button):
    def __init__(self, server: str, port: int) -> None:
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.connect((server, port))

    def right_click(self):
        self.socket.sendall(b"Rr")
        time.sleep(WAIT_TIME_SEC)

    def left_click(self):
        self.socket.sendall(b"Ll")
        time.sleep(WAIT_TIME_SEC)

    def both_click(self):
        self.socket.sendall(b"LRlr")
        time.sleep(WAIT_TIME_SEC)

    def close(self):
        self.socket.close()
