package repositories

import (
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"github.com/factly/ruspie/server/internal/infrastructure/database/actions/projects"
	"github.com/factly/ruspie/server/pkg/helper"
)

type ProjectRepository interface {
	Create(user_id, org_id uint, title string, description string, logo string) (*models.Project, error)
	List(user_id, org_id uint, pagination *helper.Pagination) ([]models.Project, uint, error)
	Update(user_id, p_id uint, title, description, logo string) (*models.Project, error)
	Delete(user_id, id uint) error
	Details(user_id, id uint) (*models.Project, error)
}

func NewProjectRepository(db database.IDatabaseService) (ProjectRepository, error) {
	pro_rep, err := projects.NewPgProjectRepository(db)
	if err != nil {
		return nil, err
	}
	return pro_rep, nil
}
