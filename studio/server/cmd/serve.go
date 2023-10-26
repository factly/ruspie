package cmd

import (
	"log"

	"github.com/factly/ruspie/server/app"
	"github.com/factly/ruspie/server/internal/infrastructure/database"
	"github.com/factly/ruspie/server/internal/infrastructure/http"
	"github.com/factly/ruspie/server/pkg/config"
	"github.com/factly/ruspie/server/pkg/logger"
	"github.com/spf13/cobra"
)

func init() {
	rootCmd.AddCommand(serveCmd)
}

var serveCmd = &cobra.Command{
	Use:   "serve",
	Short: "Starts the ruspie server",
	Run: func(cmd *cobra.Command, args []string) {
		serve()
	},
}

func serve() {
	// initially using log package to log errors because custom logger is not yet initialized
	configService := config.New()
	config, err := configService.LoadConfig()
	if err != nil {
		log.Fatal("error loading config file", "err", err.Error())
	}
	logger := logger.New()
	err = logger.SetConfig(config.GetLoggerConfig())
	if err != nil {
		log.Fatal("error setting logger config", "err", err.Error())
	}

	db := database.New()
	err = db.Connect(logger, config.GetDatabaseConfig())
	if err != nil {
		// now using custom logger to log infos, errors, warnings
		logger.Fatal("error connecting to database")
	}
	app := app.NewApp()
	app.SetLogger(logger)
	app.SetConfig(&config)
	app.SetDatabase(db)
	http.RunHttpServer(app)
}
