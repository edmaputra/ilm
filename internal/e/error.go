package e

import "errors"

var ErrNotFound = errors.New("%s with ID %s was not found")
var ErrUnknown = errors.New("unknown error")
