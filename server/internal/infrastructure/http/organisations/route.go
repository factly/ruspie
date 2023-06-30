package organisations

import (
	"github.com/factly/ruspie/server/internal/domain/repositories"
	"github.com/factly/ruspie/server/pkg/logger"
	"github.com/go-chi/chi"
)

type httpHandler struct {
	organisationRepository repositories.OrganisationRepository
	logger                 logger.ILogger
}

func (h *httpHandler) routes() chi.Router {
	router := chi.NewRouter()
	router.Get("/", h.list)
	router.Post("/", h.create)
	router.Route("/{org_id}", func(r chi.Router) {
		r.Get("/", h.details)
		r.Delete("/", h.delete)
		r.Put("/", h.update)
	})
	return router
}

func InitRoutes(r *chi.Mux, orgRepository repositories.OrganisationRepository, logger logger.ILogger) {
	httpHandler := &httpHandler{organisationRepository: orgRepository, logger: logger}
	r.Mount("/organisations", httpHandler.routes())
}
