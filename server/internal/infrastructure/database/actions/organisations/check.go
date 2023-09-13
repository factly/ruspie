package organisations

import "github.com/factly/ruspie/server/internal/domain/models"

// created_by_id should not contain same tiles

func (pg *PgOrganisationRepository) OrganisationTitleExists(title string, created_by_id *uint) bool {
	query := pg.client.Model(&models.Organisation{}).Where("title = ?", title)
	if created_by_id != nil {
		query = query.Where("created_by_id = ?", created_by_id)
	}
	err := query.First(&models.Organisation{}).Error
	return err == nil
}
