from solutions.two import position, step


def test_get_position():
    inputs = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ]

    result = position.Position()
    for input in inputs:
        current_step = step.Step.from_string(input)
        result.move(current_step)
    
    assert result.x == 15
    assert result.y == 10
