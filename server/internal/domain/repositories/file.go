package repositories

import (
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"github.com/factly/ruspie/server/internal/infrastructure/database/actions/files"
	"github.com/factly/ruspie/server/pkg/helper"
)

type FileRepository interface {
	Create(user_id, p_id uint, name, extenstion, s3_url string) (*models.File, error)
	List(user_id, p_id uint, pagination *helper.Pagination) (*[]models.File, uint, error)
	Update(user_id uint, id, name, extenstion, s3_url string) (*models.File, error)
	Delete(user_id uint, id string) error
	Details(user_id uint, id string) (*models.File, error)
}

func NewFileRepository(db database.IDatabaseService) (FileRepository, error) {
	file_repo, err := files.NewPgFileRepository(db)
	if err != nil {
		return nil, err
	}
	return file_repo, nil
}
