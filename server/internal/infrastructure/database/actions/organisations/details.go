package organisations

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/domain/models"
	"gorm.io/gorm"
)

func (pg *PgOrganisationRepository) Details(user_id, id uint) (*models.Organisation, error) {
	organisation := &models.Organisation{
		Base: models.Base{
			CreatedByID: user_id,
		},
	}
	err := pg.client.First(&organisation, id).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, &custom_errors.CustomError{Context: custom_errors.NotFound, Err: errors.New("organisation not found")}
		}
		return nil, err
	}
	return organisation, nil
}
