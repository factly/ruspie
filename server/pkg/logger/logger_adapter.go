package logger

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/factly/ruspie/server/pkg/config"
	"github.com/go-chi/chi/middleware"
	"golang.org/x/exp/slog"
)

type SlogAdapter struct {
	Logger *slog.Logger
}

func NewSlogAdapter() *SlogAdapter {
	return &SlogAdapter{}
}

// SetConfig sets the logger config
// right now it has 2 configs - 1) log level 2) output type
// log level controls what log messages are recorded or displayed based on their level
// output type controls whether the output displayed is in text or json format
func (s *SlogAdapter) SetConfig(config config.LoggerConfig) error {
	log.Println("setting up custom logger with config : log_level=", config.Level, " output_type=", config.OutputType)
	if config.OutputType == "stdout" {
		s.Logger = slog.New(slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{
			ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
				if a.Key == slog.TimeKey {
					return slog.Attr{}
				}
				return a
			},
		}))
	} else {
		s.Logger = slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
			ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
				if a.Key == slog.TimeKey {
					return slog.Attr{}
				}
				return a
			},
		}))
	}
	log.Println("successfully set up custom logger")
	return nil
}

// Info logs a message at info level
func (s *SlogAdapter) Info(message string, args ...interface{}) {
	s.Logger.Info(message, args...)
}

// Warn logs a message at warn level
func (s *SlogAdapter) Warn(message string, args ...interface{}) {
	s.Logger.Warn(message, args...)
}

// Error logs a message at error level
func (s *SlogAdapter) Error(message string, args ...interface{}) {
	s.Logger.Error(message, args...)
}

// Fatal logs a message at fatal level
func (s *SlogAdapter) Fatal(message string, args ...interface{}) {
	log.Fatal("message=", message)
}

type StructuredLogger struct {
	Logger slog.Handler
}

func (l *StructuredLogger) NewLogEntry(r *http.Request) middleware.LogEntry {
	var logFields []slog.Attr
	logFields = append(logFields, slog.String("ts", time.Now().UTC().Format(time.RFC1123)))

	if reqID := middleware.GetReqID(r.Context()); reqID != "" {
		logFields = append(logFields, slog.String("req_id", reqID))
	}

	scheme := "http"
	if r.TLS != nil {
		scheme = "https"
	}

	handler := l.Logger.WithAttrs(append(logFields,
		slog.String("http_scheme", scheme),
		slog.String("http_proto", r.Proto),
		slog.String("http_method", r.Method),
		slog.String("remote_addr", r.RemoteAddr),
		slog.String("user_agent", r.UserAgent()),
		slog.String("uri", fmt.Sprintf("%s://%s%s", scheme, r.Host, r.RequestURI))))

	entry := SlogAdapter{Logger: slog.New(handler)}

	entry.Logger.LogAttrs(r.Context(), slog.LevelInfo, "request started")

	return &entry
}

func (l *SlogAdapter) Write(status, bytes int, header http.Header, elapsed time.Duration, extra interface{}) {
	l.Logger.LogAttrs(context.TODO(), slog.LevelInfo, "request complete",
		slog.Int("resp_status", status),
		slog.Int("resp_byte_length", bytes),
		slog.Float64("resp_elapsed_ms", float64(elapsed.Nanoseconds())/1000000.0),
	)
}

func (l *SlogAdapter) Panic(v interface{}, stack []byte) {
	l.Logger.LogAttrs(context.TODO(), slog.LevelInfo, "",
		slog.String("stack", string(stack)),
		slog.String("panic", fmt.Sprintf("%+v", v)),
	)
}

func GetLogEntry(r *http.Request) *slog.Logger {
	entry := middleware.GetLogEntry(r).(*SlogAdapter)
	return entry.Logger
}

func LogEntrySetField(r *http.Request, key string, value interface{}) {
	if entry, ok := r.Context().Value(middleware.LogEntryCtxKey).(*SlogAdapter); ok {
		entry.Logger = entry.Logger.With(key, value)
	}
}

func LogEntrySetFields(r *http.Request, fields map[string]interface{}) {
	if entry, ok := r.Context().Value(middleware.LogEntryCtxKey).(*SlogAdapter); ok {
		for k, v := range fields {
			entry.Logger = entry.Logger.With(k, v)
		}
	}
}

// GetHTTPMiddleWare returns a middleware for logging http requests
func (s *SlogAdapter) GetHTTPMiddleWare() func(next http.Handler) http.Handler {
	return middleware.RequestLogger(&StructuredLogger{Logger: s.Logger.Handler()})
}
