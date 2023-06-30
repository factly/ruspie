package organisations

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
)

func (pg *PgOrganisationRepository) Create(user_id uint, title string, description string, logo string) (*models.Organisation, error) {
	exists := pg.OrganisationTitleExists(title, nil)
	if exists {
		return nil, &custom_errors.CustomError{Context: custom_errors.NameOrTitleAlreadyExists, Err: errors.New("organisation title already exists")}
	}

	newOrganisation := models.Organisation{
		Base: models.Base{
			CreatedByID: user_id,
		},
		Title:       title,
		Description: description,
		Logo:        logo,
	}

	err := pg.client.Create(&newOrganisation).Error
	if err != nil {
		return nil, err
	}

	return &newOrganisation, nil
}
