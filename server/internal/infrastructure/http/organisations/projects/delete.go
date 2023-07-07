package projects

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

func (h *httpHandler) delete(w http.ResponseWriter, r *http.Request) {
	user_id, err := helper.GetUserID(r)
	if err != nil {
		h.logger.Error("error in parsing X-User header", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User header", http.StatusUnauthorized)))
		return
	}

	p_id := helper.GetPathParamByName(r, "project_id")
	project_id, err := helper.StringToInt(p_id)
	if err != nil {
		h.logger.Error("error in parsing project_id path parameter", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid project_id path parameter", http.StatusBadRequest)))
		return
	}

	err = h.projectRepository.Delete(user_id, uint(project_id))
	if err != nil {
		h.logger.Error("error in deleting project", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.GetMessage("error in deleting project", http.StatusInternalServerError)))
		return
	}

	renderx.JSON(w, http.StatusOK, map[string]interface{}{
		"mesgage": "project deleted",
	})
	return
}
