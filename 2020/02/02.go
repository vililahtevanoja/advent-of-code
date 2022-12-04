package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

const inputFile = "input.txt"

type passwordInfo struct {
	constraint1 int
	constraint2 int
	letter      string
	pw          string
}

func main() {
	data, _ := ioutil.ReadFile(inputFile)
	lines := strings.Split(string(data), "\n")
	pwInfos := getPwInfos(lines)
	count := 0
	for _, pwInfo := range pwInfos {
		if pwInfoIsCompliantPart1(pwInfo) {
			count++
		}
	}
	fmt.Printf("Part 1: Compliant password count: %d\n", count)

	count = 0
	for _, pwInfo := range pwInfos {
		if pwInfoIsCompliantPart2(pwInfo) {
			count++
		}
	}
	fmt.Printf("Part 2: Compliant password count: %d\n", count)
}

func getPwInfos(lines []string) []passwordInfo {
	pwInfos := []passwordInfo{}
	for _, line := range lines {
		if line == "" {
			continue
		}
		pwInfo := passwordInfo{}
		parts := strings.Split(line, ":")
		pwInfo.pw = strings.Trim(parts[1], " ")
		firstSubParts := strings.Split(parts[0], " ")
		pwInfo.letter = strings.Trim(firstSubParts[1], " ")
		minMax := strings.Split(firstSubParts[0], "-")
		pwInfo.constraint1, _ = strconv.Atoi(minMax[0])
		pwInfo.constraint2, _ = strconv.Atoi(minMax[1])
		pwInfos = append(pwInfos, pwInfo)
	}
	return pwInfos
}

func pwInfoIsCompliantPart1(pwInfo passwordInfo) bool {
	count := strings.Count(pwInfo.pw, pwInfo.letter)
	return count >= pwInfo.constraint1 && count <= pwInfo.constraint1
}

func pwInfoIsCompliantPart2(pwInfo passwordInfo) bool {
	i1 := pwInfo.constraint1 - 1
	i2 := pwInfo.constraint2 - 1
	if len(pwInfo.pw) < i2 {
		return false
	}
	first := string(pwInfo.pw[i1])
	second := string(pwInfo.pw[i2])
	return (first == pwInfo.letter || second == pwInfo.letter) && !(first == pwInfo.letter && second == pwInfo.letter)
}
