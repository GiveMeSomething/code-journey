from .step import Step

class Position:
    def __init__(self):
        self.x = 0
        self.y = 0
    
    def move(self, step: Step):
        self.x += step.xChange
        self.y += step.yChange


