package projects

import "github.com/factly/ruspie/server/internal/domain/models"

func (pg *PgProjectRepository) ProjectTitleExists(title string, created_by_id *uint, org_id uint) bool {
	query := pg.client.Model(&models.Project{OrganisationID: org_id}).Where("title = ?", title)
	if created_by_id != nil {
		query = query.Where("created_by_id === ?", created_by_id)
	}
	err := query.First(&models.Project{}).Error
	return err == nil
}

func (pg *PgProjectRepository) OrganisationExists(org_id uint) bool {
	err := pg.client.Model(&models.Organisation{}).Where("id = ?", org_id).First(&models.Organisation{}).Error
	return err == nil
}
