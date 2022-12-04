package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

const inputFile = "input.txt"

func main() {
	data, _ := ioutil.ReadFile(inputFile)

	lines := strings.Split(string(data), "\n")
	ns := []int{}
	for _, line := range lines {
		if line == "" {
			continue
		}
		n, _ := strconv.Atoi(line)
		ns = append(ns, n)
	}
	for i, n1 := range ns {
		for j, n2 := range ns[i:] {
			for _, n3 := range ns[j:] {
				if n1+n2+n3 == 2020 {
					fmt.Printf("%d + %d + %d = %d\n", n1, n2, n3, n1+n2+n3)
					fmt.Printf("%d * %d * %d = %d\n", n1, n2, n3, n1*n2*n3)
				}
			}
		}
	}
}
