package httputils

type JSONResponse struct {
	Code   int          `json:"code"`
	Status string       `json:"status"`
	Data   interface{}  `json:"data,omitempty"`
	Error  *ErrorDetail `json:"error,omitempty"`
}

type ErrorDetail struct {
	Message string `json:"message,omitempty"`
}

func NewJSONResponse(code int, status string, data interface{}) JSONResponse {
	return JSONResponse{
		Code:   code,
		Status: status,
		Data:   data,
	}
}

func NewJSONErrorResponse(code int, status string, errorMessage string) JSONResponse {
	return JSONResponse{
		Code:   code,
		Status: status,
		Error: &ErrorDetail{
			Message: errorMessage,
		},
	}
}
