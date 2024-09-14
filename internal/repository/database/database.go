package database

import (
	"context"
	"database/sql"
	"log"

	"github.com/edmaputra/ilm/internal/e"
	"github.com/edmaputra/ilm/pkg/model"
	"github.com/jmoiron/sqlx"
)

type Repository struct {
	db *sqlx.DB
}

func New(db *sqlx.DB) *Repository {
	return &Repository{
		db: db,
	}
}

func (r *Repository) Get(_ context.Context, id string) (*model.Project, error) {
	var project model.Project
	err := r.db.Get(&project, "SELECT * FROM project WHERE id=$1", id)

	if err != nil {
		log.Println(err)

		if err == sql.ErrNoRows {
			return nil, e.ErrNotFound
		}

		return nil, e.ErrUnknown
	}

	return &project, nil
}
