package projects

import (
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
)

func (pg *PgProjectRepository) List(user_id uint, org_id uint, pagination *helper.Pagination) ([]models.Project, uint, error) {
	projects := make([]models.Project, 0)
	var total int64

	offset := (pagination.Page - 1) * pagination.Limit
	if offset < 0 {
		offset = 0
	}

	db := pg.client.Model(&models.Project{}).Where("created_by_id = ? AND organisation_id = ?", user_id, org_id).Order("updated_at " + pagination.Queries["sort"])

	if pagination.SearchQuery != "" {
		db = db.Where("title ILIKE ?", "%"+pagination.SearchQuery+"%")
	}

	err := db.Count(&total).Offset(offset).Limit(pagination.Limit).Find(&projects).Error

	if err != nil {
		return nil, 0, err
	}

	return projects, uint(total), nil
}
