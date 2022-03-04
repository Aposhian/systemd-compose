package convert

import (
	compose "github.com/compose-spec/compose-go/types"
	systemd "github.com/coreos/go-systemd/v22/unit"
	"log"
)

func checkForUnsupportedFields(project compose.Project) {
	if len(project.Networks) != 0 {
		log.Panicln("networks are not yet supported")
	}
	if len(project.Volumes) != 0 {
		log.Panicln("volumes are not yet supported")
	}
	if len(project.Secrets) != 0 {
		log.Panicln("secrets are not yet supported")
	}
	if len(project.Configs) != 0 {
		log.Panicln("configs are not yet supported")
	}
	if len(project.Extensions) != 0 {
		log.Panicln("extensions are not yet supported")
	}
}

func Convert(project compose.Project) [][]*systemd.UnitSection {
	checkForUnsupportedFields(project)
	units := make([][]*systemd.UnitSection, len(service))
	for _, service := range project.Services {
		// Each service will be converted into at least two unit files:
		// - a service to create the container (oneshot)
		// - a service to start the container
		convertService(service)
	}
	return nil
}
