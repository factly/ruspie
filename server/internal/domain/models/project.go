package models

type Project struct {
	Base
	Title          string        `gorm:"column:title" json:"title,required"`
	Description    string        `gorm:"column:description" json:"description,omitempty"`
	Logo           string        `gorm:"column:logo" json:"logo,omitempty"`
	OrganisationID uint          `gorm:"column:organisation_id" json:"organisation_id"`
	Organisation   *Organisation `gorm:"foreignKey:OrganisationID" json:"organisation"`
	File           []File        `gorm:"foreignKey:ProjectID" json:"projects"`
}
