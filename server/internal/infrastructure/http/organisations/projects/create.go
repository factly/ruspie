package projects

import (
	"encoding/json"
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type createProjectRequest struct {
	Title       string `json:"title" validate:"required"`
	Description string `json:"description,omitempty"`
	Logo        string `json:"logo,omitempty"`
}

func (h *httpHandler) create(w http.ResponseWriter, r *http.Request) {
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

	o_id := helper.GetPathParamByName(r, "org_id")
	org_id, err := helper.StringToInt(o_id)
	if err != nil {
		h.logger.Error("error in parsing org_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid organisation_id", http.StatusBadRequest)))
		return
	}

	requestBody := &createProjectRequest{}
	err = json.NewDecoder(r.Body).Decode(requestBody)
	if err != nil {
		h.logger.Error("error in decoding request body", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("cannot decode request body", http.StatusBadRequest)))
		return
	}

	project, err := h.projectRepository.Create(user_id, uint(org_id), requestBody.Title, requestBody.Description, requestBody.Logo)

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

	renderx.JSON(w, http.StatusCreated, project)
	return
}
