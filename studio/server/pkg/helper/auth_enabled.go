package helper

import "github.com/spf13/viper"

func AuthEnable() bool {
	var isEnabled = viper.Get("KAVACH_ENABLED")
	if isEnabled == "true" {
		return true
	}
	return false
}
