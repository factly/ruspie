package helper

import (
	"net/http"
	"strconv"
)

func GetUserID(req *http.Request) (uint, error) {
	userID := req.Header.Get("X-User")
	uID, err := strconv.Atoi(userID)
	if err != nil {
		return 0, err
	}
	return uint(uID), nil
}
