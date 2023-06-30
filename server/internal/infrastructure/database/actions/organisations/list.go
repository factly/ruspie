package organisations

import (
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
)

func (pg *PgOrganisationRepository) List(user_id uint, pagination *helper.Pagination) ([]models.Organisation, uint, error) {
	organisations := make([]models.Organisation, 0)
	var total int64

	offset := (pagination.Page - 1) * pagination.Limit
	if offset < 0 {
		offset = 0
	}

	db := pg.client.Model(&models.Organisation{}).Where("created_by_id = ?", user_id).Order("updated_at " + pagination.Queries["sort"])

	if pagination.SearchQuery != "" {
		db = db.Where("title ILIKE ?", "%"+pagination.SearchQuery+"%")
	}

	err := db.Count(&total).Offset(offset).Limit(pagination.Limit).Preload("Projects").Find(&organisations).Error

	if err != nil {
		return nil, 0, err
	}

	return organisations, uint(total), nil
}
