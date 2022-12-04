package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

const TREE = '#'
const OPEN = '.'

type slope struct {
	right int
	down  int
}

func main() {
	const inputFile = "input.txt"
	input, _ := ioutil.ReadFile(inputFile)
	pattern := strings.Split(string(input), "\n")
	slopes := []slope{
		{right: 1, down: 1},
		{right: 3, down: 1},
		{right: 5, down: 1},
		{right: 7, down: 1},
		{right: 1, down: 2}}
	treeHits := []int{}
	for _, slope := range slopes {
		numberOfTreesHit := getNumberOfTreesHit(pattern, slope)
		fmt.Printf("Number of trees hit: %d (down %d, right %d)\n", numberOfTreesHit, slope.down, slope.right)
		treeHits = append(treeHits, numberOfTreesHit)
	}
	treeProduct := getProduct(treeHits)
	fmt.Printf("Product of trees hit: %d\n", treeProduct)
}

func getNumberOfTreesHit(pattern []string, slope slope) int {
	row := 0
	column := 0
	treesHit := 0
	for row < len(pattern) {
		if hitTree(row, column, pattern) {
			treesHit++
		}
		row += slope.down
		column += slope.right
	}
	return treesHit
}

func hitTree(row int, column int, pattern []string) bool {
	moduloColumn := column % len(pattern[0])
	if pattern[row][moduloColumn] == TREE {
		return true
	}
	return false
}

func getProduct(ns []int) int {
	nProduct := ns[0]
	for _, n := range ns[1:] {
		nProduct = nProduct * n
	}
	return nProduct
}
