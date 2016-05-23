package main

import (
	"fmt"
	"math"
)

type ErrNegativeSqrt float64

func (e ErrNegativeSqrt) Error() string {
	return fmt.Sprintf("cannot Sqrt negative number: %f", float64(e))
}

func Sqrt(x float64) (float64, error) {
	z := 1.0
	zn := 0.0

	if x < 0.0 {
		return 0.0, ErrNegativeSqrt(x)
	}

	for {
		zn = z - (z*z-x)/(2*z)

		if math.Abs(zn-z) < 0.000001 {
			break
		}

		z = zn
	}

	return z, nil
}

func main() {
	fmt.Println(Sqrt(2))
	fmt.Println(Sqrt(-2))
}
