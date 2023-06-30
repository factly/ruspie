package organisations

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"gorm.io/gorm"
)

type PgOrganisationRepository struct {
	client *gorm.DB
}

func NewPgOrganisationRepository(db database.IDatabaseService) (*PgOrganisationRepository, error) {
	db_client, ok := db.GetClient().(*gorm.DB)
	if !ok {
		return nil, &custom_errors.CustomError{Context: custom_errors.InvalidDatabaseClient, Err: errors.New("cannot cast database client to gorm.DB")}
	}
	return &PgOrganisationRepository{client: db_client}, nil
}
