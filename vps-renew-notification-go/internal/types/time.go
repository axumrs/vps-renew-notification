package types

import (
	"strings"
	"time"
)

type Date time.Time

const dateFormat = "2006-01-02"

func (d *Date) UnmarshalJSON(data []byte) error {

	loc, err := time.LoadLocation("Asia/Shanghai")
	if err != nil {
		return err
	}
	date, err := time.ParseInLocation(dateFormat, strings.Trim(string(data), `"`), loc)
	if err != nil {
		return err
	}
	*d = Date(date)
	return nil
}

func (d Date) MarshalJSON() ([]byte, error) {
	buf := make([]byte, 0, len(dateFormat)+2)
	buf = append(buf, '"')
	buf = time.Time(d).AppendFormat(buf, dateFormat)
	buf = append(buf, '"')
	return buf, nil
}
