package database

import (
	"github.com/factly/ruspie/server/pkg/config"
	"github.com/factly/ruspie/server/pkg/logger"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	gormLogger "gorm.io/gorm/logger"
)

type IDatabaseService interface {
	Connect(logger logger.ILogger, cfg config.DatabaseConfig) error
	GetClient() interface{}
}

type PgDatabaseAdapter struct {
	Client *gorm.DB
}

func New() IDatabaseService {
	return &PgDatabaseAdapter{}
}

func (p *PgDatabaseAdapter) Connect(logger logger.ILogger, cfg config.DatabaseConfig) error {

	logger.Info("connecting to database")
	// dsn (Data source name) is the connection string for the database
	dsn := "host=" + cfg.Host + " user=" + cfg.Username + " password=" + cfg.Password + " dbname=ruspie port=" + cfg.Port + " sslmode=" + cfg.SSLMode
	var err error
	p.Client, err = gorm.Open(postgres.Open(dsn), &gorm.Config{
		Logger: gormLogger.Default.LogMode(gormLogger.Info),
	})
	if err != nil {
		return err
	}
	logger.Info("successfully connected to database")
	return nil
}

func (p *PgDatabaseAdapter) GetClient() interface{} {
	return p.Client
}
