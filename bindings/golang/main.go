package main

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/tonyhb/rs-templating/bindings/golang/ffitemplating"
)

func main() {
	vars := map[string]interface{}{
		"name": "tester MCTESTYFACE",
		"now":  time.Now(),
	}

	inspected := ffitemplating.Variables(template)
	fmt.Println("vars", inspected)

	byt, _ := json.Marshal(vars)
	result := ffitemplating.Execute(template, byt)
	fmt.Println("executed", result)
}

const template = `Hi {{ name | title | trim }}, the current date is {{ now | date(format="%Y-%m-%d") }}.`
