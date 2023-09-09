package organisations

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/repositories"
	"github.com/factly/ruspie/server/internal/infrastructure/http/organisations/projects"
	"github.com/factly/ruspie/server/pkg/logger"
	"github.com/factly/x/renderx"
	"github.com/go-chi/chi"
)

type httpHandler struct {
	organisationRepository repositories.OrganisationRepository
	logger                 logger.ILogger
}

func (h *httpHandler) routes(server_context repositories.ServerRepoContext, logger logger.ILogger) chi.Router {
	router := chi.NewRouter()
	router.Get("/", h.list)
	router.Post("/", h.create)
	router.Route("/{org_id}/", func(r chi.Router) {
		r.Mount("/projects", projects.InitRoutes(server_context, logger))
		r.Mount("/hello", hello())
		r.Get("/", h.details)
		r.Delete("/", h.delete)
		r.Put("/", h.update)
	})
	return router
}

func InitRoutes(r *chi.Mux, server_context repositories.ServerRepoContext, logger logger.ILogger) {
	httpHandler := &httpHandler{organisationRepository: server_context.GetOrganisationRepository(), logger: logger}
	r.Mount("/organisations", httpHandler.routes(server_context, logger))
}

func hello() chi.Router {
	r := chi.NewRouter()
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		renderx.JSON(w, http.StatusOK, "Hello")
	})
	return r
}
