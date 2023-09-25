package helper

import (
	"math/rand"
	"regexp"

	"github.com/google/uuid"
)

func Generate_UUID() string {
	return removeSpecialCharacters(generateRandomAlphaString(8) + uuid.New().String())
}

func removeSpecialCharacters(s string) string {
	// Define a regular expression to match non-alphanumeric characters.
	regex := regexp.MustCompile("[^a-zA-Z0-9]+")

	// Use the regular expression to replace non-alphanumeric characters with an empty string.
	return regex.ReplaceAllString(s, "")
}

func generateRandomAlphaString(length int) string {
	const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	result := make([]byte, length)
	for i := 0; i < length; i++ {
		result[i] = charset[rand.Intn(len(charset))]
	}
	return string(result)
}
