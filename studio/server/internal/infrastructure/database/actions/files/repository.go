package files

import (
	"errors"

	"github.com/factly/ruspie/server/internal/domain/custom_errors"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"gorm.io/gorm"
)

type PgFileRepository struct {
	client *gorm.DB
}

func NewPgFileRepository(db database.IDatabaseService) (*PgFileRepository, error) {
	db_client, ok := db.GetClient().(*gorm.DB)
	if !ok {
		return nil, &custom_errors.CustomError{Context: custom_errors.InvalidDatabaseClient, Err: errors.New("cannot cast database client to gorm.DB")}
	}
	return &PgFileRepository{client: db_client}, nil
}
