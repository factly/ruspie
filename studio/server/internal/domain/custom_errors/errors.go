package custom_errors

type CustomError struct {
	Err     error
	Context CustomErrorType
}

type CustomErrorType string

const (
	InnerEntityNotFound      CustomErrorType = "inner entity not found"
	NotFound                 CustomErrorType = "not found"
	NameOrTitleAlreadyExists CustomErrorType = "already exists"
	InvalidDatabaseClient    CustomErrorType = "invalid database client"
)

// Error implements error
func (c *CustomError) Error() string {
	return c.Err.Error()
}
