package projects

import (
	"github.com/factly/ruspie/server/internal/domain/repositories"
	"github.com/factly/ruspie/server/internal/infrastructure/http/organisations/projects/files"
	"github.com/factly/ruspie/server/pkg/logger"
	"github.com/go-chi/chi"
)

type httpHandler struct {
	projectRepository repositories.ProjectRepository
	logger            logger.ILogger
}

func (h *httpHandler) routes(server_context repositories.ServerRepoContext, logger logger.ILogger) chi.Router {
	router := chi.NewRouter()
	router.Get("/", h.list)
	router.Post("/", h.create)
	router.Route("/{project_id}", func(r chi.Router) {
		r.Get("/", h.details)
		r.Delete("/", h.delete)
		r.Put("/", h.update)
		r.Mount("/files", files.InitRoutes(server_context, logger))
	})
	return router
}

func InitRoutes(server_context repositories.ServerRepoContext, logger logger.ILogger) chi.Router {
	httpHandler := &httpHandler{projectRepository: server_context.GetProjectRepository(), logger: logger}
	return httpHandler.routes(server_context, logger)
}
