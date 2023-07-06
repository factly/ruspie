package files

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type response struct {
	Count   uint           `json:"count"`
	Files   *[]models.File `json:"files"`
	Message string         `json:"message"`
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

	p_id := pagination.Queries["p_id"]
	if p_id == "" {
		h.logger.Error("p_id query param is missing", "error", "missing p_id query param")
		errorx.Render(w, errorx.Parser(errorx.GetMessage("missing p_id query param", http.StatusBadRequest)))
		return
	}

	pro_id, err := helper.StringToInt(p_id)
	if err != nil {
		h.logger.Error("error in parsing p_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid p_id", http.StatusBadRequest)))
		return
	}

	res := &response{}
	res.Files, res.Count, err = h.fileRepository.List(user_id, uint(pro_id), pagination)
	if err != nil {
		h.logger.Error("error listing projects", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("cannot list projects", http.StatusInternalServerError)))
		return
	}

	res.Message = "fetched files successfully"

	renderx.JSON(w, http.StatusOK, res)
	return
}
