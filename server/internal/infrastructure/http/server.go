package http

import (
	"fmt"
	"net/http"

	"github.com/factly/ruspie/server/app"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/go-chi/cors"
)

func RunHttpServer(app *app.App) {
	logger := app.GetLogger()
	cfg := app.GetConfig()
	_ = app.GetDatabase()
	logger.Info("Starting HTTP server on PORT: " + cfg.GetServerConfig().Port)

	router := chi.NewRouter()
	router.Use(cors.Handler(cors.Options{
		// AllowedOrigins:   []string{"https://foo.com"}, // Use this to allow specific origin hosts
		AllowedOrigins: []string{"http://*"},
		// AllowOriginFunc:  func(r *http.Request, origin string) bool { return true },
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Accept", "Authorization", "Content-Type", "X-CSRF-Token", "X-User"},
		ExposedHeaders:   []string{"Link"},
		AllowCredentials: false,
		MaxAge:           300, // Maximum value not ignored by any of major browsers
	}))

	router.Use(middleware.RequestID)
	router.Use(middleware.RealIP)
	router.Use(logger.GetHTTPMiddleWare())
	router.Get("/hello", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Welcome to Ruspie!"))
		return
	})

	err := http.ListenAndServe(fmt.Sprintf(":%s", cfg.GetServerConfig().Port), router)
	if err != nil {
		logger.Fatal(fmt.Sprintf("error starting HTTP server: %s", err.Error()))
	}
}
