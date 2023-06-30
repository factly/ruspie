package helper

import (
	"errors"
	"net/http"
	"strconv"
)

type Pagination struct {
	// Page, Limit, SearchQuery are mandatory fields for pagination
	// Queries is a map of optional fields for pagination Ex: sort
	Page        int
	Limit       int
	SearchQuery string
	Queries     map[string]string
}

// GetPagination returns pagination object
func GetPagination(r *http.Request) (*Pagination, error) {
	pagination := &Pagination{}
	var err error
	page := r.URL.Query().Get("page")
	if page == "" {
		pagination.Page = 1
	} else {
		pagination.Page, err = strconv.Atoi(page)
		if err != nil {
			return nil, errors.New("Invalid page parameter")
		}
	}

	limit := r.URL.Query().Get("limit")
	if limit == "" {
		pagination.Limit = 12
	} else {
		pagination.Limit, err = strconv.Atoi(limit)
		if err != nil {
			return nil, errors.New("Invalid limit parameter")
		}
	}

	pagination.SearchQuery = r.URL.Query().Get("search_query")
	pagination.Queries = map[string]string{}

	sort := r.URL.Query().Get("sort")
	if sort != "" {
		if sort != "asc" && sort != "desc" {
			return nil, errors.New("Invalid sort parameter")
		}
		pagination.Queries["sort"] = sort
	} else {
		pagination.Queries["sort"] = "desc"
	}

	org_id := r.URL.Query().Get("org_id")
	if org_id != "" {
		pagination.Queries["org_id"] = org_id
	}

	return pagination, nil
}
