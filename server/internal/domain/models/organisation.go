package models

type Organisation struct {
	Base
	Title       string    `gorm:"column:title" json:"name,required"`
	Description string    `gorm:"column:description" json:"desc,omitempty"`
	Logo        string    `gorm:"column:logo" json:"logo,omitempty"`
	Projects    []Project `gorm:"foreignKey:OrganisationID" json:"projects"`
}
