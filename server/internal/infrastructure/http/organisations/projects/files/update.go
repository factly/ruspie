package files

import (
	"encoding/json"
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type updateRequest struct {
	Name      string `json:"name"`
	Extension string `json:"extension"`
	S3Url     string `json:"s3_url"`
}

func (h *httpHandler) update(w http.ResponseWriter, r *http.Request) {

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

	var updateReq updateRequest
	err = json.NewDecoder(r.Body).Decode(&updateReq)

	if err != nil {
		h.logger.Error("error in decoding request body", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid request body", http.StatusBadRequest)))
		return
	}

	updateFile, err := h.fileRepository.Update(uint(user_id), f_id, updateReq.Name, updateReq.Extension, updateReq.S3Url)
	if err != nil {
		h.logger.Error("error in updating project", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			} else if customErr.Context == custom_errors.NameOrTitleAlreadyExists {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusConflict)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.GetMessage("error in updating project", http.StatusInternalServerError)))
		return
	}

	renderx.JSON(w, http.StatusOK, updateFile)
	return

}
