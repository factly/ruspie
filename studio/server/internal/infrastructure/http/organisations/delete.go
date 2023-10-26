package organisations

import (
	"net/http"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/pkg/helper"
	"github.com/factly/x/errorx"
	"github.com/factly/x/renderx"
)

func (h *httpHandler) delete(w http.ResponseWriter, r *http.Request) {
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
		h.logger.Error("error in parsing org_id path parameter", "error", err.Error())
		errorx.Render(w, errorx.Parser(errorx.GetMessage("invalid org_id path parameter", http.StatusBadRequest)))
		return
	}

	err = h.organisationRepository.Delete(user_id, uint(org_id))
	if err != nil {
		h.logger.Error("error in deleting organisation", "error", err.Error())
		if customErr, ok := err.(*custom_errors.CustomError); ok {
			if customErr.Context == custom_errors.NotFound {
				errorx.Render(w, errorx.Parser(errorx.GetMessage(err.Error(), http.StatusNotFound)))
				return
			}
			errorx.Render(w, errorx.Parser(errorx.InternalServerError()))
			return
		}
		errorx.Render(w, errorx.Parser(errorx.GetMessage("error in deleting organisation", http.StatusInternalServerError)))
		return
	}

	renderx.JSON(w, http.StatusOK, map[string]interface{}{
		"mesgage": "organisation deleted",
	})
	return
}
