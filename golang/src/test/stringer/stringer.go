package main

import "fmt"

//go:generate stringer -type hoge
type hoge int

const (
	Alice hoge = iota
	Bob
	Carol
	Dave
	Eve
	Frank
	Godzilla
)

func main() {

	var test hoge

	for test = Alice; test <= Godzilla; test++ {
		fmt.Println(test)
	}
}
