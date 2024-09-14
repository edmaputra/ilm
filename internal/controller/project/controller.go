package project

import (
	"context"

	"github.com/edmaputra/ilm/pkg/model"
)

type projectRepository interface {
	Get(ctx context.Context, id string) (*model.Project, error)
}

type Controller struct {
	repo projectRepository
}

func New(repo projectRepository) *Controller {
	return &Controller{
		repo,
	}
}

func (c *Controller) Get(ctx context.Context, id string) (*model.Project, error) {
	project, err := c.repo.Get(ctx, id)

	if err != nil {
		return nil, err
	}

	return project, nil
}
