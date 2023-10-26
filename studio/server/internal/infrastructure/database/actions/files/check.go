package files

import "github.com/factly/ruspie/server/internal/domain/models"

func (pg *PgFileRepository) FileNameExists(name string, created_by_id *uint) bool {
	query := pg.client.Model(&models.File{}).Where("name= ?", name)
	if created_by_id != nil {
		query = query.Where("created_by_id != ?", created_by_id)
	}
	err := query.First(&models.File{}).Error
	return err == nil
}
