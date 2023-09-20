package files

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/pkg/helper"
)

func (pg *PgFileRepository) Create(user_id, p_id uint, name, extenstion, s3_url string) (*models.File, error) {
	exists := pg.FileNameExists(name, nil)
	if exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NameOrTitleAlreadyExists, Err: errors.New("file name already exists")}
	}

	newFile := models.File{}
	newFile.CreatedByID = user_id
	newFile.Name = name
	newFile.Extension = extenstion
	newFile.S3Url = s3_url
	newFile.ProjectID = p_id
	newFile.ID = helper.Generate_UUID()
	err := pg.client.Create(&newFile).Error
	if err != nil {
		return nil, err
	}
	return &newFile, nil
}
