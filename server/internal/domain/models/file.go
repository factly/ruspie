package models

type File struct {
	Base
	Name string `gorm:"column:name;not null" json:"name"`
	// extension of the file can only be either parquet or csv setting default to parquet
	Extension string   `gorm:"column:extension;not null;default:parquet" json:"extension"`
	S3Url     string   `gorm:"column:s3_url;not null" json:"s3_url"`
	ProjectID uint     `gorm:"column:project_id;not null" json:"project_id"`
	Project   *Project `gorm:"foreignKey:ProjectID" json:"project"`
}
