package files

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgFileRepository) Delete(user_id uint, id uint) error {
	file := &models.File{}
	file.ID = id
	err := pg.client.Where("created_by_id = ? AND id = ?", user_id, id).First(&file).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("file not found")}
		}
		return err
	}
	err = pg.client.Delete(&file, id).Error

	if err != nil {
		return err
	}
	return nil
}
