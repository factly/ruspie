package organisations

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type listOrgs struct {
	Count         uint                  `json:"count"`
	Organisations []models.Organisation `json:"organisations"`
	Message       string                `json:"message"`
}

func (h *httpHandler) list(w http.ResponseWriter, r *http.Request) {
	user_id, err := helper.GetUserID(r)
	if err != nil {
		h.logger.Error("error decoding request body", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User Header", http.StatusUnauthorized)))
		return
	}

	pagination, err := helper.GetPagination(r)
	if err != nil {
		h.logger.Error("error in parsing pagination", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusBadRequest)))
		return
	}

	response := &listOrgs{}
	response.Organisations, response.Count, err = h.organisationRepository.List(user_id, pagination)
	if err != nil {
		h.logger.Error("error listing organisations", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("cannot list organisations", http.StatusInternalServerError)))
		return
	}
	response.Message = "organisations fetched successfully"
	renderx.JSON(w, http.StatusOK, response)
	return
}
