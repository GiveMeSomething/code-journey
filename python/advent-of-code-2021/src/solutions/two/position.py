from .step import Step

class Position:
    def __init__(self):
        self.x = 0
        self.y = 0
        self.aim = 0

    def move(self, step: Step):
        self.x += step.xChange
        self.y += step.yChange

    def move_with_aim(self, step: Step):
        self.aim += step.yChange
        if step.xChange == 0:
            return
        self.x += step.xChange
        self.y += step.xChange * self.aim


