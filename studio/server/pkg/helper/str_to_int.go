package helper

import "strconv"

func StringToInt(s string) (int, error) {
	return strconv.Atoi(s)
}
