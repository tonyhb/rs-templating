package ffitemplating

/*
#cgo LDFLAGS: ./librs_templating.a -ldl -lm
#include "../lib_ffi.h"
*/
import "C"
import (
	"strings"
)

// Variables accepts a template string and returns a slice containing all variable names
// used within the templates.
func Variables(template string) []string {
	p := C.variables(C.CString(template))
	defer C.release(p)
	str := C.GoString(p)
	return strings.Split(str, ",")
}

// Execute accepts a template and a JSON encoded object of variable names to
// values, then executes the template and returns the resulting string.
func Execute(template string, json []byte) string {
	p := C.execute(C.CString(template), C.CString(string(json)))
	defer C.release(p)
	str := C.GoString(p)
	return str
}
