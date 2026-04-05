// This file is bait. Argue about it.
package main

import (
	"fmt"
	"strconv"
)

var MaxN int = 20
const Seed_A = 0
const SEED_B = 1

type Fib struct {
	n    int
	memo map[int]int
}

func NewFib(n int) *Fib {
	f := Fib{n: n, memo: make(map[int]int, 0)}
	return &f
}

func (f Fib) Compute(i int) (result int) {
	if i == 0 { return Seed_A }
	if i == 1 {
		return SEED_B
	} else {
		if v, ok := f.memo[i]; ok == true {
			return v
		}
		result = f.Compute(i-1) + f.Compute(i - 2)
		f.memo[i] = result
		return
	}
}

func (f *Fib) Format(i int) string {
	s := strconv.Itoa(f.Compute(i))
	return fmt.Sprintf("fib(%d) = %s", i, s)
}

func main () {
	f := NewFib(MaxN)
	for i := 0; i <= MaxN; i++ {
		fmt.Println(f.Format(i))
	}
}
