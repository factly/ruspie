package projects

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgProjectRepository) Update(user_id, o_id, p_id uint, title, description, logo string) (*models.Project, error) {
	exists := pg.ProjectTitleExists(title, &user_id, o_id)
	if exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NameOrTitleAlreadyExists, Err: errors.New("project title already exists")}
	}

	updateMap := map[string]interface{}{}

	if title != "" {
		updateMap["title"] = title
	}

	if description != "" {
		updateMap["description"] = description
	}

	if logo != "" {
		updateMap["logo"] = logo
	}

	project := &models.Project{}

	err := pg.client.Model(&models.Project{}).Where("created_by_id = ? AND id = ?", user_id, p_id).Updates(updateMap).Preload("Organisation").First(&project).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("project not found")}
		}
		return nil, err
	}

	return project, nil
}

func (pg *PgProjectRepository) ChangeOrganisation(
	user_id, project_id, new_org_id, old_org_id uint,
) (*models.Project, error) {
	exists := pg.OrganisationExists(new_org_id)

	if !exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.InnerEntityNotFound, Err: errors.New("new organisation id not found")}
	}

	project := models.Project{}
	err := pg.client.Model(&models.Project{}).Where("created_by_id = ? AND id = ? AND organisation_id = ?", user_id, project_id, old_org_id).First(&project).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("project not found")}
		}
		return nil, err
	}
	project.OrganisationID = new_org_id

	err = pg.client.Save(&project).Error
	if err != nil {
		return nil, err
	}

	return &project, nil
}
