package main

import (
	"fmt"
	"golang.org/x/tour/tree"
	"reflect"
	"sort"
)

func subWalk(t *tree.Tree, ch chan int) {

	ch <- t.Value

	if t.Left != nil {
		subWalk(t.Left, ch)
	}

	if t.Right != nil {
		subWalk(t.Right, ch)
	}
}

// Walk walks the tree t sending all values
// from the tree to the channel ch.
func Walk(t *tree.Tree, ch chan int) {

	subWalk(t, ch)

	close(ch)
}

// Same determines whether the trees
// t1 and t2 contain the same values.
func Same(t1, t2 *tree.Tree) bool {

	var tree1 []int
	var tree2 []int

	ch1 := make(chan int)
	ch2 := make(chan int)

	go Walk(t1, ch1)
	go Walk(t2, ch2)

	for c1 := range ch1 {
		tree1 = append(tree1, c1)
	}

	for c2 := range ch2 {
		tree2 = append(tree2, c2)
	}

	sort.Sort(sort.IntSlice(tree1))
	sort.Sort(sort.IntSlice(tree2))

	if reflect.DeepEqual(tree1, tree2) {
		return true
	} else {
		return false
	}

}

func main() {

	ch := make(chan int)

	go Walk(tree.New(1), ch)

	for c := range ch {
		fmt.Println(c)
	}

	fmt.Println(Same(tree.New(1), tree.New(1)))
	fmt.Println(Same(tree.New(1), tree.New(2)))
}
