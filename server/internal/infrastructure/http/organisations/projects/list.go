package projects

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type listProjects struct {
	Count    uint             `json:"count"`
	Projects []models.Project `json:"projects"`
	Message  string           `json:"message"`
}

func (h *httpHandler) list(w http.ResponseWriter, r *http.Request) {

	var err error
	var user_id uint = 1
	if helper.AuthEnable() {
		user_id, err = helper.GetUserID(r)
		if err != nil {
			h.logger.Error("error in parsing X-User header", "error", err.Error())
			errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User Header", http.StatusUnauthorized)))
			return
		}
	}

	pagination, err := helper.GetPagination(r)
	if err != nil {
		h.logger.Error("error in parsing pagination", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusBadRequest)))
		return
	}

	// o_id := pagination.Queries["org_id"]
	// if o_id == "" {
	// 	h.logger.Error("org_id query param is missing", "error", "missing org_id query param")
	// 	errorx.Render(w, errorx.Parser(errorx.GetMessage("missing org_id query param", http.StatusBadRequest)))
	// 	return
	// }
	o_id := helper.GetPathParamByName(r, "org_id")
	org_id, err := helper.StringToInt(o_id)
	if err != nil {
		h.logger.Error("error in parsing org_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid org_id", http.StatusBadRequest)))
		return
	}
	response := &listProjects{}
	response.Projects, response.Count, err = h.projectRepository.List(user_id, uint(org_id), pagination)
	if err != nil {
		h.logger.Error("error listing projects", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("cannot list projects", http.StatusInternalServerError)))
		return
	}
	response.Message = "projects fetched successfully"

	renderx.JSON(w, http.StatusOK, response)
	return
}
