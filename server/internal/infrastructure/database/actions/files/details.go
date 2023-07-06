package files

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgFileRepository) Details(user_id, p_id uint) (*models.File, error) {
	file := &models.File{}
	err := pg.client.Where("created_by_id = ? AND id = ?", user_id, p_id).Preload("Project").First(&file).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("file with given id not found")}
		}
		return nil, err
	}
	return file, nil
}
