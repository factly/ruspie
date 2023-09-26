package files

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgFileRepository) Update(user_id uint, f_id, name, extenstion, s3_url string) (*models.File, error) {
	exists := pg.FileNameExists(name, &user_id)
	if exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NameOrTitleAlreadyExists, Err: errors.New("project title already exists")}
	}

	updateMap := map[string]interface{}{}

	if name != "" {
		updateMap["name"] = name
	}

	if extenstion != "" {
		updateMap["extenstion"] = extenstion
	}

	if s3_url != "" {
		updateMap["s3_url"] = s3_url
	}

	file := &models.File{}
	err := pg.client.Model(&models.File{}).Where("created_by_id = ? AND id = ?", user_id, f_id).Updates(updateMap).First(&file).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("file not found")}
		}
		return nil, err
	}

	return file, nil
}
