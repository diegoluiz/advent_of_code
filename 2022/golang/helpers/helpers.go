package helpers

import (
	"fmt"
	"time"
)

func StartDay(day string) time.Time {
	time := time.Now()

	fmt.Printf("%v - Starting day [%v]\n", time.Format("2006-01-02 15:04:05.000000"), day)

	return time
}

func EndDay(start time.Time) {
	fmt.Printf("%v - Finished. Time Elapsed %v\n", time.Now().Format("2006-01-02 15:04:05.000000"), time.Since(start))
}

func Start() time.Time {
	time := time.Now()
	fmt.Printf("%v - Starting AoC 2022\n", time.Format("2006-01-02 15:04:05.000000"))
	return time
}

func End(start time.Time) {
	fmt.Printf("%v - Finished AoC. Time Elapsed %v\n", time.Now().Format("2006-01-02 15:04:05.000000"), time.Since(start))
}
