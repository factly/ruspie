package files

import (
	"encoding/json"
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type request struct {
	Name      string `json:"name"`
	Extension string `json:"extension"`
	S3Url     string `json:"s3_url"`
}

func (h *httpHandler) create(w http.ResponseWriter, r *http.Request) {
	user_id, err := helper.GetUserID(r)
	if err != nil {
		h.logger.Error("error in parsing X-User header", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User Header", http.StatusUnauthorized)))
		return
	}

	p_id := helper.GetPathParamByName(r, "project_id")
	project_id, err := helper.StringToInt(p_id)
	if err != nil {
		h.logger.Error("error in parsing org_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid project_id", http.StatusBadRequest)))
		return
	}

	requestBody := &request{}
	err = json.NewDecoder(r.Body).Decode(requestBody)
	if err != nil {
		h.logger.Error("error in decoding request body", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("cannot decode request body", http.StatusBadRequest)))
		return
	}

	file, err := h.fileRepository.Create(user_id, uint(project_id), requestBody.Name, requestBody.Extension, requestBody.S3Url)

	if err != nil {
		h.logger.Error("error in creating project", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NameOrTitleAlreadyExists {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusConflict)))
				return
			} else if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusInternalServerError)))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
		return
	}

	renderx.JSON(w, http.StatusCreated, file)
	return
}
