package memory

import (
	"context"
	"sync"

	"github.com/edmaputra/ilm/internal/repository"
	"github.com/edmaputra/ilm/pkg/model"
)

type Repository struct {
	sync.RWMutex
	data map[string]*model.Project
}

func New() *Repository {
	return &Repository{
		data: map[string]*model.Project{
			"1": {
				ID:          "1",
				Name:        "test",
				Description: "test",
			},
		},
	}
}

func (r *Repository) Get(_ context.Context, id string) (*model.Project, error) {
	r.RLock()
	defer r.RUnlock()

	project, ok := r.data[id]

	if !ok {
		return nil, repository.ErrNotFound
	}

	return project, nil
}

func (r *Repository) Put(_ context.Context, id string, project *model.Project) error {
	r.Lock()
	defer r.Unlock()

	r.data[id] = project
	return nil

}
