package main

import (
	"fmt"
	"math"
)

func Sqrt(x float64) float64 {

	z := 1.0
	zn := 0.0
	i := 0

	for {
		i++
		zn = z - (z*z-x)/(2*z)
		if math.Abs(zn-z) < 0.000001 {
			break
		}
		z = zn
	}

	fmt.Println(i)

	return zn
}

func main() {
	fmt.Println(
		Sqrt(2),
		math.Sqrt(2),
	)
}
