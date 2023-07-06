package files

import (
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
)

func (pg *PgFileRepository) List(user_id, p_id uint, pagination *helper.Pagination) (*[]models.File, uint, error) {
	files := make([]models.File, 0)
	var total int64

	offset := (pagination.Page - 1) * pagination.Limit
	if offset < 0 {
		offset = 0
	}

	db := pg.client.Model(&models.File{}).Where("created_by_id = ? AND project_id = ?", user_id, p_id).Order("updated_at " + pagination.Queries["sort"])

	if pagination.SearchQuery != "" {
		db = db.Where("name ILIKE ?", "%"+pagination.SearchQuery+"%")
	}
	err := db.Count(&total).Offset(offset).Limit(pagination.Limit).Find(&files).Error

	if err != nil {
		return nil, uint(total), err
	}
	return &files, uint(total), nil
}
