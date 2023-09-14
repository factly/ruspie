package projects

import (
	"encoding/json"
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

type updateRequest struct {
	Title       string `json:"title,omitempty"`
	Description string `json:"description,omitempty"`
	Logo        string `json:"logo,omitempty"`
}

func (h *httpHandler) update(w http.ResponseWriter, r *http.Request) {

	userID, err := helper.GetUserID(r)
	if err != nil {
		h.logger.Error("error in parsing X-User header", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User header", http.StatusUnauthorized)))
		return
	}

	p_id := helper.GetPathParamByName(r, "project_id")
	project_id, err := helper.StringToInt(p_id)
	if err != nil {
		h.logger.Error("error in parsing project_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid project_id", http.StatusBadRequest)))
		return
	}
	o_id := helper.GetPathParamByName(r, "org_id")
	org_id, err := helper.StringToInt(o_id)
	if err != nil {
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

	updatedProject, err := h.projectRepository.Update(uint(userID), uint(org_id), uint(project_id), updateReq.Title, updateReq.Description, updateReq.Logo)
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

	renderx.JSON(w, http.StatusOK, updatedProject)
	return
}

type changeOrgIdRequest struct {
	NewOrgId string `json:"new_org_id"`
}

func (h *httpHandler) changeOrgId(w http.ResponseWriter, r *http.Request) {

	userID, err := helper.GetUserID(r)
	if err != nil {
		h.logger.Error("error in parsing X-User header", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid X-User header", http.StatusUnauthorized)))
		return
	}

	p_id := helper.GetPathParamByName(r, "project_id")
	project_id, err := helper.StringToInt(p_id)
	if err != nil {
		h.logger.Error("error in parsing project_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid project_id", http.StatusBadRequest)))
		return
	}

	var changeOrgIdReq changeOrgIdRequest
	err = json.NewDecoder(r.Body).Decode(&changeOrgIdReq)
	if err != nil {
		h.logger.Error("error in decoding request body", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid request body", http.StatusBadRequest)))
		return
	}
	o_id := helper.GetPathParamByName(r, "org_id")
	org_id, err := helper.StringToInt(o_id)
	if err != nil {
		h.logger.Error("error in parsing org_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid organisation_id", http.StatusBadRequest)))
		return
	}
	newOrgId, err := helper.StringToInt(changeOrgIdReq.NewOrgId)
	if err != nil {
		h.logger.Error("error in parsing project_id", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid new_org_id", http.StatusBadRequest)))
		return
	}

	if org_id == newOrgId {
		errorx.Render(w, errorx.Parser(errorx.GetMessage("new_org_id is same as the previous org_id", http.StatusBadRequest)))
		return
	}

	updatedProject, err := h.projectRepository.ChangeOrganisation(userID, uint(project_id), uint(newOrgId), uint(org_id))
	if err != nil {
		h.logger.Error("error in updating project", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			} else if customErr.Context == custom_errors.InnerEntityNotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusBadRequest)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.GetMessage("error in updating project", http.StatusInternalServerError)))
		return
	}

	renderx.JSON(w, http.StatusOK, updatedProject)
	return
}
