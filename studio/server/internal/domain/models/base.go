package models

import (
	"time"

	"gorm.io/gorm"
)

// Base is the base model for all models it has all the common fields
type Base struct {
	ID          uint            `gorm:"primaryKey" json:"id"`
	CreatedAt   time.Time       `json:"createdAt"`
	UpdatedAt   time.Time       `json:"updatedAt"`
	DeletedAt   *gorm.DeletedAt `gorm:"index" json:"deleted_at" swaggertype:"primitive,string"`
	CreatedByID uint            `gorm:"column:created_by_id" json:"created_by_id"`
	UpdatedByID uint            `gorm:"column:updated_by_id" json:"updated_by_id"`
}
