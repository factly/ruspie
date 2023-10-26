package repositories

type ServerRepoContext interface {
	GetOrganisationRepository() OrganisationRepository
	GetProjectRepository() ProjectRepository
	GetFileRepository() FileRepository
}

type ServerRepoContextImpl struct {
	OrganisationRepository OrganisationRepository
	ProjectRepository      ProjectRepository
	FileRepository         FileRepository
}

func (s ServerRepoContextImpl) GetOrganisationRepository() OrganisationRepository {
	return s.OrganisationRepository
}

func (s ServerRepoContextImpl) GetProjectRepository() ProjectRepository {
	return s.ProjectRepository
}

func (s ServerRepoContextImpl) GetFileRepository() FileRepository {
	return s.FileRepository
}

func NewServerRepoContext(orgRepo OrganisationRepository, projectRepo ProjectRepository, fileRepo FileRepository) ServerRepoContext {
	return ServerRepoContextImpl{
		OrganisationRepository: orgRepo,
		ProjectRepository:      projectRepo,
		FileRepository:         fileRepo,
	}
}
