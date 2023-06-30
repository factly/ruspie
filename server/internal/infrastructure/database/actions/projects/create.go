package projects

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
)

func (pg *PgProjectRepository) Create(user_id, org_id uint, title, description, logo string) (*models.Project, error) {
	exists := pg.ProjectTitleExists(title, nil)
	if exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NameOrTitleAlreadyExists, Err: errors.New("project title already exists")}
	}

	exists = pg.OrganisationExists(org_id)
	if !exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("organisation with give id not found")}
	}

	project := &models.Project{
		Base: models.Base{
			CreatedByID: user_id,
		},
		Title:          title,
		Description:    description,
		Logo:           logo,
		OrganisationID: org_id,
	}

	err := pg.client.Create(&project).Error
	if err != nil {
		return nil, err
	}

	return project, nil
}
