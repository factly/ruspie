package files

import (
	"github.com/factly/ruspie/server/internal/domain/repositories"
	"github.com/factly/ruspie/server/pkg/logger"
	"github.com/go-chi/chi"
)

type httpHandler struct {
	fileRepository repositories.FileRepository
	logger         logger.ILogger
}

func (h *httpHandler) routes() chi.Router {
	router := chi.NewRouter()
	router.Get("/", h.list)
	router.Post("/", h.create)
	router.Route("/{file_id}", func(r chi.Router) {
		r.Get("/", h.details)
		r.Delete("/", h.delete)
		r.Put("/", h.update)
	})
	return router
}

func InitRoutes(r *chi.Mux, fileRepository repositories.FileRepository, logger logger.ILogger) {
	httpHandler := &httpHandler{fileRepository: fileRepository, logger: logger}
	r.Mount("/files", httpHandler.routes())
}
