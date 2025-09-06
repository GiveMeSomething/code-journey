
from utils.util_number import try_atoi


COMMAND_FORWARD = "forward"
COMMAND_UP = "up"
COMMAND_DOWN = "down"


class Step:
    def __init__(self, xChange, yChange):
        self.xChange = xChange
        self.yChange = yChange

    def from_string(s: str) -> 'Step':
        command, commandValue = s.split(" ", 1)
        commandValueNum, err = try_atoi(commandValue)
        if err is not None:
            raise Exception(f"Invalid command value: {commandValue}")
        
        if command == COMMAND_FORWARD:
            return Step(commandValueNum, 0)
        if command == COMMAND_UP:
            return Step(0, -commandValueNum)
        if command == COMMAND_DOWN:
            return Step(0, commandValueNum)
        
        raise Exception(f"Invalid command: {command}")

