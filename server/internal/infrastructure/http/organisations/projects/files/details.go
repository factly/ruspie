package files

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

func (h *httpHandler) details(w http.ResponseWriter, r *http.Request) {

	user_id, err := helper.GetUserID(r)
	if err != nil {
		h.logger.Error("error in parsing X-User header", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User header", http.StatusUnauthorized)))
		return
	}

	f_id := helper.GetPathParamByName(r, "file_id")

	if f_id == "" {
		h.logger.Error("error in parsing project_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid project_id", http.StatusBadRequest)))
		return
	}

	file, err := h.fileRepository.Details(uint(user_id), f_id)
	if err != nil {
		h.logger.Error("error in fetching project", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.GetMessage("error in fetching project", http.StatusInternalServerError)))
		return
	}

	renderx.JSON(w, http.StatusOK, file)
	return
}
