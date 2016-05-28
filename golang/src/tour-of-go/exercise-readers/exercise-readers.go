package main

import "golang.org/x/tour/reader"

type MyReader struct{}

func (r MyReader) Read(b []byte) (int, error) {

	i := 0

	for ; i < len(b); i++ {
		b[i] = 'A'
	}

	return i, nil
}

func main() {
	reader.Validate(MyReader{})
}
