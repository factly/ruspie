package projects

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgProjectRepository) Delete(user_id, id uint) error {
	project := &models.Project{
		Base: models.Base{
			CreatedByID: user_id,
		},
	}
	err := pg.client.Where("created_by_id = ? AND id = ?", user_id, id).First(&project).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("project not found")}
		}
		return err
	}
	err = pg.client.Delete(&project, id).Error

	if err != nil {
		return err
	}
	return nil
}
