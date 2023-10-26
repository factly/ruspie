package app

import (
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"github.com/factly/ruspie/server/pkg/config"
	"github.com/factly/ruspie/server/pkg/logger"
)

type App struct {
	logger   logger.ILogger
	config   *config.ConfigPort
	database database.IDatabaseService
}

func NewApp() *App {
	return &App{}
}

func (a *App) SetLogger(logger logger.ILogger) {
	a.logger = logger
}

func (a *App) SetConfig(config *config.ConfigPort) {
	a.config = config
}

func (a *App) SetDatabase(database database.IDatabaseService) {
	a.database = database
}

func (a *App) GetLogger() logger.ILogger {
	return a.logger
}

func (a *App) GetConfig() config.ConfigPort {
	return *a.config
}

func (a *App) GetServerConfig() config.ServerConfig {
	return (*a.config).GetServerConfig()
}

func (a *App) GetDatabase() database.IDatabaseService {
	return a.database
}
