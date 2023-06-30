package repositories

import (
	"github.com/factly/ruspie/server/internal/domain/models"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"github.com/factly/ruspie/server/internal/infrastructure/database/actions/organisations"
	"github.com/factly/ruspie/server/pkg/helper"
)

type OrganisationRepository interface {
	Create(user_id uint, title string, description string, logo string) (*models.Organisation, error)
	List(user_id uint, pagination *helper.Pagination) ([]models.Organisation, uint, error)
	Update(user_id, org_id uint, title, description, logo string) (*models.Organisation, error)
	Delete(user_id, id uint) error
	Details(user_id, id uint) (*models.Organisation, error)
}

func NewOrganisationRepository(db database.IDatabaseService) (OrganisationRepository, error) {
	org_rep, err := organisations.NewPgOrganisationRepository(db)
	if err != nil {
		return nil, err
	}
	return org_rep, nil
}
