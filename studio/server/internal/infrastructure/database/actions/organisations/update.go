package organisations

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgOrganisationRepository) Update(user_id, org_id uint, title, description, logo string) (*models.Organisation, error) {
	exists := pg.OrganisationTitleExists(title, &user_id)
	if exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NameOrTitleAlreadyExists, Err: errors.New("organisation title already exists")}
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

	organisation := &models.Organisation{}

	err := pg.client.Model(&models.Organisation{}).Where("created_by_id = ? AND id = ?", user_id, org_id).Updates(updateMap).First(&organisation).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("organisation not found")}
		}
		return nil, err
	}

	return organisation, nil
}
