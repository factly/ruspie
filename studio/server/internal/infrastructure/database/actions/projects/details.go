package projects

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgProjectRepository) Details(user_id, o_id, p_id uint) (*models.Project, error) {
	project := &models.Project{}
	err := pg.client.Where("created_by_id = ? AND id = ? AND organisation_id = ?", user_id, p_id, o_id).Preload("Organisation").Preload("Files").First(&project).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("project with given id not found")}
		}
		return nil, err
	}
	return project, nil
}
