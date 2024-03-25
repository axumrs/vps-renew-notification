package pkg

import "strings"

// IsEmptyStr 是否为空字符串
func IsEmptyStr(s string) bool {
	return len(strings.TrimSpace(s)) == 0
}
