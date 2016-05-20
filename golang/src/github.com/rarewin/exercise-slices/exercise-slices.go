package main

import "golang.org/x/tour/pic"

func Pic(dx, dy int) [][]uint8 {

	ret := make([][]uint8, dy)

	for i, _ := range ret {

		ar := make([]uint8, dx)
		for x := 0; x < dx; x++ {
			ar[x] = uint8(x * i)
		}
		ret[i] = ar
	}

	return ret
}

func main() {
	pic.Show(Pic)
}
