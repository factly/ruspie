package projects

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"gorm.io/gorm"
)

type PgProjectRepository struct {
	client *gorm.DB
}

func NewPgProjectRepository(db database.IDatabaseService) (*PgProjectRepository, error) {
	client, ok := db.GetClient().(*gorm.DB)
	if !ok {
		return nil, &custom_errors.CustomError{Context: custom_errors.InvalidDatabaseClient, Err: errors.New("cannot cast database client to gorm.DB")}
	}
	return &PgProjectRepository{client: client}, nil
}
