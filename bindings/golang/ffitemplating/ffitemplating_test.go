package ffitemplating

import (
	"encoding/json"
	"testing"
)

func BenchmarkExecute(b *testing.B) {
	tpl := "Hi, {{ name | trim | upper }}."
	vars, _ := json.Marshal(map[string]interface{}{"name": "bench "})
	b.ResetTimer()
	for n := 0; n < b.N; n++ {
		res := Execute(tpl, vars)
		if res != "Hi, BENCH." {
			b.Fatalf("invalid template result: %s", res)
		}
	}
}
