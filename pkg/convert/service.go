package convert

import (
	"log"
	compose "github.com/compose-spec/compose-go/types"
	systemd "github.com/coreos/go-systemd/v22/unit"
)

func getUnits(service compose.ServiceConfig) {
	systemd.UnitSection{
		Section: "Unit",
		Entries: append([]systemd.UnitEntry,
			systemd.UnitEntry{
				Name: "Description",
				Value: "",
			}
		),
	}
}