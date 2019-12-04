package main

import (
    "fmt"
    "time"
)

func tarai(x int, y int, z int) int {
    if x <= y {
        return y
    }
    return tarai(
        tarai(x - 1, y, z),
        tarai(y - 1, z, x),
        tarai(z - 1, x, y),
    );
}

func main() {
    start := time.Now()
    result := tarai(15, 5, 0)
    end := time.Now()

    fmt.Printf("%d\n", result)
    fmt.Printf("%fs\n", (end.Sub(start)).Seconds())
}

