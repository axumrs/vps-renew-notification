package pkg

import "github.com/rs/xid"

// NewID 生成新的ID
func NewID() string {
	return xid.New().String()
}
