package helper

import (
	"net/http"

	"github.com/go-chi/chi"
)

func GetPathParamByName(r *http.Request, name string) string {
	return chi.URLParam(r, name)
}
