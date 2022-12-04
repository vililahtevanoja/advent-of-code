package main

import (
	"strings"
	"testing"
)

func TestSolution1(t *testing.T) {
	input := `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
	const expected = 7
	pattern := strings.Split(input, "\n")
	actual := getNumberOfTreesHit(pattern, slope{3, 1})
	if actual != expected {
		t.Fatalf("expected: %d, actual: %d", expected, actual)
	}
}

func TestSolution2(t *testing.T) {
	input := `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`
	const expected = 336
	slopes := []slope{
		{right: 1, down: 1},
		{right: 3, down: 1},
		{right: 5, down: 1},
		{right: 7, down: 1},
		{right: 1, down: 2}}
	pattern := strings.Split(input, "\n")
	hits := []int{}
	for _, slope := range slopes {
		numberOfTreesHit := getNumberOfTreesHit(pattern, slope)
		t.Logf("Number of trees hit: %d (down %d, right %d)\n", numberOfTreesHit, slope.down, slope.right)
		hits = append(hits, numberOfTreesHit)
	}

	actual := getProduct(hits)
	if actual != expected {
		t.Fatalf("expected: %d, actual: %d", expected, actual)
	}
}
