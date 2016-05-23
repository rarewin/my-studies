package main

import (
	"io"
	"os"
	"strings"
)

type rot13Reader struct {
	r io.Reader
}

func (r13r rot13Reader) Read(b []byte) (int, error) {

	i := 0
	cb := make([]byte, len(b))

	n, err := r13r.r.Read(cb)

	if err != nil {
		return 0, err
	}

	for ; i < n; i++ {
		if 'a' <= cb[i] && cb[i] <= 'm' {
			b[i] = cb[i] + 13
		} else if 'n' <= cb[i] && cb[i] <= 'z' {
			b[i] = cb[i] - 13
		} else if 'A' <= cb[i] && cb[i] <= 'M' {
			b[i] = cb[i] + 13
		} else if 'N' <= cb[i] && cb[i] <= 'Z' {
			b[i] = cb[i] - 13
		} else {
			b[i] = cb[i]
		}
	}

	return n, nil
}

func main() {
	s := strings.NewReader("Lbh penpxrq gur pbqr!")
	r := rot13Reader{s}
	io.Copy(os.Stdout, &r)
}
