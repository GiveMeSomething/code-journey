package five

type Point struct {
	X int
	Y int
}

type VentLine struct {
	From       Point
	To         Point
	IsDiagonal bool
}

func (ventLine *VentLine) Init(fromX, fromY, toX, toY int) *VentLine {
	isDiagonal := fromX != toX && fromY != toY
	ventLine.From = Point{
		X: fromX,
		Y: fromY,
	}
	ventLine.To = Point{
		X: toX,
		Y: toY,
	}
	ventLine.IsDiagonal = isDiagonal
	return ventLine
}
