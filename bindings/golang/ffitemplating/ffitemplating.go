//go:build !musl

package ffitemplating

/*
#cgo linux,amd64 LDFLAGS: ${SRCDIR}/librs_templating_x86_linux_gnu.a -ldl -lm
#cgo linux,arm64 LDFLAGS: ${SRCDIR}/librs_templating_aarch64_linux_gnu.a -ldl -lm
#cgo darwin,amd64 LDFLAGS: ${SRCDIR}/librs_templating_x86_darwin.a -ldl -lm
#cgo darwin,arm64 LDFLAGS: ${SRCDIR}/librs_templating_aarch64_darwin.a -ldl -lm
#include "./lib_ffi.h"
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
