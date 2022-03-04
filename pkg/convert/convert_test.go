package convert

import (
	"testing"
	compose "github.com/compose-spec/compose-go/types"
)

func Test_Convert(t *testing.T) {
	units := Convert(compose.Project{
		Services: compose.Services{},
		Networks: compose.Networks{},
		Volumes:  compose.Volumes{},
		Secrets:  compose.Secrets{},
		Configs:  compose.Configs{},
	})
	if units == nil {
		t.Fatal()
	}
}