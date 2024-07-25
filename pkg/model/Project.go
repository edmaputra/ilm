package model

type Project struct {
	ID             string `json:"id"`
	Name           string `json:"name"`
	Description    string `json:"description"`
	Flow_stages_id string `json:"flow_stages_id"`
	Created_at     uint   `json:"created_at"`
	Created_by     string `json:"created_by"`
	Updated_at     uint   `json:"updated_at"`
}
