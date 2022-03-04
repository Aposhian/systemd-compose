package convert

import (
	compose "github.com/compose-spec/compose-go/types"
	systemd "github.com/coreos/go-systemd/v22/unit"
)

func Convert(project compose.Project) []*systemd.UnitSection {
	return nil
}
