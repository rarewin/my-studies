package main

import (
	"fmt"
	"sync"
)

type Fetcher interface {
	// Fetch returns the body of URL and
	// a slice of URLs found on that page.
	Fetch(url string, ch chan fetchResult)
}

var read_urls = make(map[string]bool)
var mtx sync.Mutex

func Crawl(url string, depth int, fetcher Fetcher) {

	var ch = make(chan fetchResult, 20)

	if depth <= 0 {
		return
	}

	go fetcher.Fetch(url, ch)

	for f := range ch {

		if f.err != nil {
			fmt.Println(f.err)
			return
		}

		if f.body != "" {

			fmt.Printf("found: %s %q\n", url, f.body)

			for _, u := range f.urls {
				Crawl(u, depth-1, fetcher)
			}
		}
	}

}

func main() {
	Crawl("http://golang.org/", 4, fetcher)
}

// fakeFetcher is Fetcher that returns canned results.
type fakeFetcher map[string]*fakeResult

type fetchResult struct {
	body string
	urls []string
	err  error
}

type fakeResult struct {
	body string
	urls []string
}

func (f fakeFetcher) Fetch(url string, ch chan fetchResult) {

	mtx.Lock()
	if _, ok := read_urls[url]; ok == true {
		ch <- fetchResult{"", nil, nil}
		close(ch)
		mtx.Unlock()
		return
	}

	read_urls[url] = true
	mtx.Unlock()

	if res, ok := f[url]; ok {
		ch <- fetchResult{res.body, res.urls, nil}
	} else {
		ch <- fetchResult{"", nil, fmt.Errorf("not found %s", url)}
	}

	close(ch)
}

// fetcher is a populated fakeFetcher.
var fetcher = fakeFetcher{
	"http://golang.org/": &fakeResult{
		"The Go Programming Language",
		[]string{
			"http://golang.org/pkg/",
			"http://golang.org/cmd/",
		},
	},
	"http://golang.org/pkg/": &fakeResult{
		"Packages",
		[]string{
			"http://golang.org/",
			"http://golang.org/cmd/",
			"http://golang.org/pkg/fmt/",
			"http://golang.org/pkg/os/",
		},
	},
	"http://golang.org/pkg/fmt/": &fakeResult{
		"Package fmt",
		[]string{
			"http://golang.org/",
			"http://golang.org/pkg/",
		},
	},
	"http://golang.org/pkg/os/": &fakeResult{
		"Package os",
		[]string{
			"http://golang.org/",
			"http://golang.org/pkg/",
		},
	},
}
