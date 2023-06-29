package models

type Organisation struct {
	Base
	Name        string    `gorm:"column:name" json:"name,required"`
	Description string    `gorm:"column:desc" json:"desc,omitempty"`
	Logo        string    `gorm:"column:logo" json:"logo,omitempty"`
	Projects    []Project `gorm:"foreignKey:OrganisationID" json:"projects"`
}
