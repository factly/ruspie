package models

import (
	"time"

	"gorm.io/gorm"
)

type File struct {
	ID          uint            `gorm:"primaryKey" json:"id"`
	CreatedAt   time.Time       `json:"createdAt"`
	UpdatedAt   time.Time       `json:"updatedAt"`
	DeletedAt   *gorm.DeletedAt `gorm:"index" json:"deleted_at" swaggertype:"primitive,string"`
	CreatedByID uint            `gorm:"column:created_by_id" json:"created_by_id"`
	UpdatedByID uint            `gorm:"column:updated_by_id" json:"updated_by_id"`
	Name        string          `gorm:"column:name;not null" json:"name"`
	// extension of the file can only be either parquet or csv setting default to parquet
	Extension string   `gorm:"column:extension;not null;default:csv" json:"extension"`
	S3Url     string   `gorm:"column:s3_url;not null" json:"s3_url"`
	ProjectID uint     `gorm:"column:project_id;not null" json:"project_id"`
	Project   *Project `gorm:"foreignKey:ProjectID" json:"project"`
}
