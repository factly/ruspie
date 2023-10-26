package logger

import (
	"net/http"

	"github.com/factly/ruspie/server/pkg/config"
)

type ILogger interface {
	Info(message string, args ...interface{})
	Warn(message string, args ...interface{})
	Error(message string, args ...interface{})
	GetHTTPMiddleWare() func(next http.Handler) http.Handler
	Fatal(message string, args ...interface{})
	// Panic(message string, args ...interface{})

	SetConfig(config config.LoggerConfig) error
}

func New() ILogger {
	return NewSlogAdapter()
}
