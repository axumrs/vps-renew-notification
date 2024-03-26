package types

import (
	"encoding/json"
	"testing"
	"time"
)

func TestDateJson(t *testing.T) {
	d := Date(time.Now())

	buf, err := json.Marshal(d)
	t.Logf("v: %s, err: %v\n", buf, err)

	type Foo struct {
		Dateline Date `json:"dateline"`
	}
	buf, err = json.Marshal(Foo{Dateline: Date(time.Now())})
	t.Logf("v: %s, err: %v\n", buf, err)
}
