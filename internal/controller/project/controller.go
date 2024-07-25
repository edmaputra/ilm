package project

import (
	"context"
	"errors"

	"github.com/edmaputra/ilm/internal/repository"
	"github.com/edmaputra/ilm/pkg/model"
)

var ErrNotFound = errors.New("not found")

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

	if err != nil && errors.Is(err, repository.ErrNotFound) {
		return nil, ErrNotFound
	}

	return project, nil
}
