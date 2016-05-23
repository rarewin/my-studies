package main

import (
	"golang.org/x/tour/wc"
	"strings"
)

func WordCount(s string) map[string]int {

	r := make(map[string]int)

	for _, v := range strings.Fields(s) {
		r[v] = r[v] + 1
	}

	return r
}

func main() {
	wc.Test(WordCount)
}
